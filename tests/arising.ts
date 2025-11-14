import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import BN from "bn.js";
import {
  AccountMeta,
  ComputeBudgetProgram,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";

import { Arising } from "../target/types/arising";
import { BASE_ENERGY, CIVILIZATIONS, TOKEN_METADATA_PROGRAM_ID } from "./constants";
import {
  deriveCharacterMintPda,
  deriveCharacterPda,
  deriveCharacterTokenPda,
  deriveCollectionMintPda,
  deriveCollectionTokenPda,
  deriveMintStatePda,
  deriveResourceMintPda,
  findMasterEditionPda,
  findMetadataPda,
} from "./helpers";

const toNumber = (value: BN | number): number =>
  typeof value === "number" ? value : value.toNumber();

const genderFor = (civilizationIndex: number, characterId: number) =>
  (civilizationIndex + characterId) % 2 === 0 ? { male: {} } : { female: {} };

const genderKey = (gender: { male?: {}; female?: {} }) =>
  "male" in gender ? "male" : "female";

describe("arising program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.arising as Program<Arising>;
  const wallet = provider.wallet as anchor.Wallet & { payer: anchor.web3.Keypair };
  const mintStatePda = deriveMintStatePda(program.programId);
  const collectionMintPda = deriveCollectionMintPda(program.programId);
  const collectionTokenPda = deriveCollectionTokenPda(program.programId);
  const collectionMetadataPda = findMetadataPda(
    collectionMintPda,
    TOKEN_METADATA_PROGRAM_ID
  );
  const collectionEditionPda = findMasterEditionPda(
    collectionMintPda,
    TOKEN_METADATA_PROGRAM_ID
  );

  type ProgramMethods = Program<Arising>["methods"];
  type MethodsBuilderFor<T extends keyof ProgramMethods> = ReturnType<ProgramMethods[T]>;
  type StrictAccounts<T extends keyof ProgramMethods> = Parameters<
    MethodsBuilderFor<T>["accountsStrict"]
  >[0];
  type InitializeAccounts = StrictAccounts<"initialize">;
  type MintCharacterAccounts = StrictAccounts<"mintCharacter">;
  type UpdateCivilizationCapAccounts = StrictAccounts<"updateCivilizationCap">;

  const buildMintAccounts = (
    civilizationIndex: number,
    characterId: number,
    authority: anchor.web3.PublicKey
  ): MintCharacterAccounts => {
    const character = deriveCharacterPda(civilizationIndex, characterId, program.programId);
    const characterMint = deriveCharacterMintPda(
      civilizationIndex,
      characterId,
      program.programId
    );
    const characterTokenAccount = deriveCharacterTokenPda(
      civilizationIndex,
      characterId,
      program.programId
    );
    const metadata = findMetadataPda(characterMint, TOKEN_METADATA_PROGRAM_ID);
    const masterEdition = findMasterEditionPda(characterMint, TOKEN_METADATA_PROGRAM_ID);

    const accounts: MintCharacterAccounts = {
      character,
      authority,
      mintState: mintStatePda,
      characterMint,
      characterTokenAccount,
      metadata,
      masterEdition,
      collectionMint: collectionMintPda,
      collectionMetadata: collectionMetadataPda,
      collectionMasterEdition: collectionEditionPda,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
    };

    return accounts;
  };

  const mintCharacterTx = async (
    civilization: (typeof CIVILIZATIONS)[number],
    characterId: number,
    authority: Keypair = wallet.payer
  ) => {
    const gender = genderFor(civilization.index, characterId);
    const accounts = buildMintAccounts(civilization.index, characterId, authority.publicKey);
    let method = program.methods
      .mintCharacter(civilization.variant, characterId, gender)
      .accountsStrict(accounts)
      .preInstructions([
        ComputeBudgetProgram.setComputeUnitLimit({
          units: 350_000,
        }),
      ]);

    if (!authority.publicKey.equals(wallet.publicKey)) {
      method = method.signers([authority]);
    }

    return method.rpc();
  };

  const updateCapTx = async (
    civilization: (typeof CIVILIZATIONS)[number],
    cap: number,
    authority: Keypair = wallet.payer
  ) => {
    const accounts: UpdateCivilizationCapAccounts = {
      mintState: mintStatePda,
      authority: authority.publicKey,
    };

    let method = program.methods
      .updateCivilizationCap(civilization.variant, cap)
      .accountsStrict(accounts);

    if (!authority.publicKey.equals(wallet.publicKey)) {
      method = method.signers([authority]);
    }

    return method.rpc();
  };

  type ResourceKey = "gold" | "log" | "plank" | "woodenSword";

  const ensureAta = async (mint: PublicKey, owner: PublicKey = wallet.publicKey) => {
    const ata = getAssociatedTokenAddressSync(mint, owner, true);
    const info = await provider.connection.getAccountInfo(ata);
    if (!info) {
      const tx = new Transaction().add(
        createAssociatedTokenAccountInstruction(wallet.publicKey, ata, owner, mint)
      );
      await provider.sendAndConfirm(tx, []);
    }
    return ata;
  };

  const remainingAccountsForResources = async (
    resources: ResourceKey[],
    owner: PublicKey = wallet.publicKey
  ): Promise<AccountMeta[]> => {
    const metas: AccountMeta[] = [];
    for (const resource of resources) {
      const mint = deriveResourceMintPda(resource, program.programId);
      const ata = await ensureAta(mint, owner);
      metas.push({ pubkey: mint, isSigner: false, isWritable: true });
      metas.push({ pubkey: ata, isSigner: false, isWritable: true });
    }
    return metas;
  };

  const advanceClockBySeconds = async (seconds: number) => {
    const rpc: any = provider.connection;
    if (typeof rpc._rpcRequest !== "function") {
      throw new Error("warpSlot RPC method is unavailable");
    }
    const currentSlot = await provider.connection.getSlot();
    const slotsToAdvance = Math.max(1, seconds * 4);
    await rpc._rpcRequest("warpSlot", [currentSlot + slotsToAdvance]);
  };

  const advanceClockByHours = async (hours: number) => {
    await advanceClockBySeconds(hours * 60 * 60);
  };

  const getCharacterPda = (
    civilization: (typeof CIVILIZATIONS)[number],
    characterId: number
  ) => deriveCharacterPda(civilization.index, characterId, program.programId);

  const mintNextCharacter = async (civilization: (typeof CIVILIZATIONS)[number]) => {
    const mintState = await program.account.characterMintState.fetch(mintStatePda);
    const nextId = mintState.civilizationMints[civilization.index].nextId;
    await mintCharacterTx(civilization, nextId);
    return {
      characterId: nextId,
      character: getCharacterPda(civilization, nextId),
    };
  };

  const startQuestTx = async (
    civilization: (typeof CIVILIZATIONS)[number],
    characterId: number,
    questId: number
  ) => {
    await program.methods
      .startQuest(civilization.variant, characterId, questId)
      .accountsStrict({
        character: getCharacterPda(civilization, characterId),
        authority: wallet.publicKey,
      })
      .rpc();
  };

  const claimQuestTx = async (
    civilization: (typeof CIVILIZATIONS)[number],
    characterId: number,
    questId: number,
    resourceRewards: ResourceKey[] = []
  ) => {
    const remaining = await remainingAccountsForResources(resourceRewards);
    await program.methods
      .claimQuest(civilization.variant, characterId, questId)
      .accountsStrict({
        character: getCharacterPda(civilization, characterId),
        authority: wallet.publicKey,
        mintState: mintStatePda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .remainingAccounts(remaining)
      .rpc();
  };

  const startRecipeTx = async (
    civilization: (typeof CIVILIZATIONS)[number],
    characterId: number,
    recipeId: number,
    resourceCosts: ResourceKey[]
  ) => {
    const remaining = await remainingAccountsForResources(resourceCosts);
    await program.methods
      .startRecipe(civilization.variant, characterId, recipeId)
      .accountsStrict({
        character: getCharacterPda(civilization, characterId),
        authority: wallet.publicKey,
        mintState: mintStatePda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .remainingAccounts(remaining)
      .rpc();
  };

  const claimRecipeTx = async (
    civilization: (typeof CIVILIZATIONS)[number],
    characterId: number,
    recipeId: number,
    resourceRewards: ResourceKey[]
  ) => {
    const remaining = await remainingAccountsForResources(resourceRewards);
    await program.methods
      .claimRecipe(civilization.variant, characterId, recipeId)
      .accountsStrict({
        character: getCharacterPda(civilization, characterId),
        authority: wallet.publicKey,
        mintState: mintStatePda,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .remainingAccounts(remaining)
      .rpc();
  };

  const tokenAmountFor = async (mint: PublicKey) => {
    const ata = await ensureAta(mint);
    const account = await getAccount(provider.connection, ata);
    return Number(account.amount);
  };

  const expectAnchorError = async (fn: () => Promise<unknown>, code: string) => {
    try {
      await fn();
      expect.fail(`Expected ${code} error but transaction succeeded`);
    } catch (error) {
      const anchorError = error as AnchorError;
      if (!anchorError.error?.errorCode?.code) {
        throw error;
      }
      expect(anchorError.error.errorCode.code).to.equal(code);
    }
  };

  before(async () => {

    const initializeAccounts: InitializeAccounts = {
      mintState: mintStatePda,
      authority: wallet.publicKey,
      collectionMint: collectionMintPda,
      collectionTokenAccount: collectionTokenPda,
      collectionMetadata: collectionMetadataPda,
      collectionMasterEdition: collectionEditionPda,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    };

    await program.methods
      .initialize(
        "Arising Collection",
        "ARISE",
        "https://mints.playarising.com/collection.json"
      )
      .accountsStrict(initializeAccounts)
      .rpc();
  });

  it("updates and enforces civilization caps", async () => {
    for (const civilization of CIVILIZATIONS) {
      await updateCapTx(civilization, 5);
    }

    for (const civilization of CIVILIZATIONS) {
      for (let characterId = 1; characterId <= 5; characterId += 1) {
        await mintCharacterTx(civilization, characterId);
      }

      const firstCharacter = await program.account.character.fetch(
        deriveCharacterPda(civilization.index, 1, program.programId)
      );
      const firstGender = genderFor(civilization.index, 1);

      expect(firstCharacter.civilization).to.deep.equal(civilization.variant);
      expect(firstCharacter.civilizationCharacterId).to.equal(1);
      expect(firstCharacter.gender[genderKey(firstGender)]).to.not.be.undefined;
      expect(firstCharacter.class).to.equal(null);
      expect(toNumber(firstCharacter.experience)).to.equal(0);

      expect(toNumber(firstCharacter.stats.might)).to.equal(civilization.stats.might);
      expect(toNumber(firstCharacter.stats.speed)).to.equal(civilization.stats.speed);
      expect(toNumber(firstCharacter.stats.intellect)).to.equal(
        civilization.stats.intellect
      );

      expect(toNumber(firstCharacter.attributes.atk)).to.equal(
        civilization.attributes.atk
      );
      expect(toNumber(firstCharacter.attributes.def)).to.equal(
        civilization.attributes.def
      );
      expect(toNumber(firstCharacter.attributes.magAtk)).to.equal(
        civilization.attributes.magAtk
      );
      expect(toNumber(firstCharacter.attributes.magDef)).to.equal(
        civilization.attributes.magDef
      );
      expect(toNumber(firstCharacter.attributes.range)).to.equal(
        civilization.attributes.range
      );
      expect(toNumber(firstCharacter.attributes.rate)).to.equal(
        civilization.attributes.rate
      );

      expect(toNumber(firstCharacter.energy)).to.equal(BASE_ENERGY);
      expect(toNumber(firstCharacter.lastEnergyRefill)).to.equal(
        toNumber(firstCharacter.createdAt)
      );
    }

    for (const civilization of CIVILIZATIONS) {
      await expectAnchorError(
        () => mintCharacterTx(civilization, 6),
        "CivilizationCapReached"
      );
    }

    const unauthorized = Keypair.generate();

    await expectAnchorError(
      () => updateCapTx(CIVILIZATIONS[0], 6, unauthorized),
      "Unauthorized"
    );

    for (const civilization of CIVILIZATIONS) {
      await updateCapTx(civilization, 6);
      await mintCharacterTx(civilization, 6);
    }
  });

  it("runs quests, forge, and craft flows end-to-end", async () => {
    const civilization = CIVILIZATIONS[0];
    await updateCapTx(civilization, 50);

    const gatherer = await mintNextCharacter(civilization);
    const logMint = deriveResourceMintPda("log", program.programId);
    const plankMint = deriveResourceMintPda("plank", program.programId);
    const goldMint = deriveResourceMintPda("gold", program.programId);
    const swordMint = deriveResourceMintPda("woodenSword", program.programId);

    // Gather raw materials and gold via quests
    await startQuestTx(civilization, gatherer.characterId, 2); // Woodcutting
    await advanceClockByHours(1);
    await claimQuestTx(civilization, gatherer.characterId, 2, ["log"]);

    await startQuestTx(civilization, gatherer.characterId, 1); // Caravan Guard
    await advanceClockByHours(1);
    await claimQuestTx(civilization, gatherer.characterId, 1, ["gold"]);

    await startQuestTx(civilization, gatherer.characterId, 2);
    await advanceClockByHours(1);
    await claimQuestTx(civilization, gatherer.characterId, 2, ["log"]);

    expect(await tokenAmountFor(logMint)).to.equal(2);
    expect(await tokenAmountFor(goldMint)).to.equal(100);

    // Forge logs into planks
    const forger = await mintNextCharacter(civilization);
    for (let i = 0; i < 2; i += 1) {
      await startRecipeTx(civilization, forger.characterId, 1, ["log"]);
      await advanceClockByHours(1);
      await claimRecipeTx(civilization, forger.characterId, 1, ["plank"]);
    }

    expect(await tokenAmountFor(logMint)).to.equal(0);
    expect(await tokenAmountFor(plankMint)).to.equal(2);

    // Craft a weapon using planks and gold
    const crafter = await mintNextCharacter(civilization);
    await startRecipeTx(civilization, crafter.characterId, 2, ["plank", "gold"]);
    await advanceClockByHours(2);
    await claimRecipeTx(civilization, crafter.characterId, 2, ["woodenSword"]);

    expect(await tokenAmountFor(plankMint)).to.equal(0);
    expect(await tokenAmountFor(goldMint)).to.equal(50);
    expect(await tokenAmountFor(swordMint)).to.equal(1);

    const crafterAccount = await program.account.character.fetch(crafter.character);
    expect(toNumber(crafterAccount.energy)).to.equal(BASE_ENERGY - 7);
    expect(crafterAccount.currentRecipe).to.equal(null);
  });
});
