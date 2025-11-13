import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import BN from "bn.js";
import {
  ComputeBudgetProgram,
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

import { Arising } from "../target/types/arising";
import { BASE_ENERGY, CIVILIZATIONS, TOKEN_METADATA_PROGRAM_ID } from "./constants";
import {
  deriveCharacterMintPda,
  deriveCharacterPda,
  deriveCharacterTokenPda,
  deriveCollectionMintPda,
  deriveCollectionTokenPda,
  deriveMintStatePda,
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
  type InitializeAccounts = Parameters<
    ReturnType<ProgramMethods["initialize"]>["accounts"]
  >[0];
  type MintCharacterAccounts = Parameters<
    ReturnType<ProgramMethods["mintCharacter"]>["accounts"]
  >[0];

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

    return {
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
      .accounts(accounts)
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
    let method = program.methods
      .updateCivilizationCap(civilization.variant, cap)
      .accounts({
        mintState: mintStatePda,
        authority: authority.publicKey,
      });

    if (!authority.publicKey.equals(wallet.publicKey)) {
      method = method.signers([authority]);
    }

    return method.rpc();
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

    const initializeAccounts = {
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
    } as InitializeAccounts;

    await program.methods
      .initialize(
        "Arising Collection",
        "ARISE",
        "https://mints.playarising.com/collection.json"
      )
      .accounts(initializeAccounts)
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
});
