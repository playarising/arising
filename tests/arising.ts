import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import BN from "bn.js";
import { ComputeBudgetProgram, SystemProgram } from "@solana/web3.js";
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

describe("arising defaults", () => {
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

  CIVILIZATIONS.forEach((civilization, index) => {
    it(`mints ${civilization.display} and checks defaults`, async () => {
      const characterId = 1;
      const character = deriveCharacterPda(
        civilization.index,
        characterId,
        program.programId
      );
      const characterMint = deriveCharacterMintPda(
        civilization.index,
        characterId,
        program.programId
      );
      const characterTokenAccount = deriveCharacterTokenPda(
        civilization.index,
        characterId,
        program.programId
      );
      const metadata = findMetadataPda(characterMint, TOKEN_METADATA_PROGRAM_ID);
      const masterEdition = findMasterEditionPda(
        characterMint,
        TOKEN_METADATA_PROGRAM_ID
      );

      const gender = index % 2 === 0 ? { male: {} } : { female: {} };

      const mintAccounts = {
        character,
        authority: wallet.publicKey,
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
      } as MintCharacterAccounts;

      await program.methods
        .mintCharacter(civilization.variant, characterId, gender)
        .accounts(mintAccounts)
        .preInstructions([
          ComputeBudgetProgram.setComputeUnitLimit({
            units: 350_000,
          }),
        ])
        .rpc();

      const account = await program.account.character.fetch(character);

      expect(account.civilization).to.deep.equal(civilization.variant);
      expect(account.civilizationCharacterId).to.equal(characterId);
      expect(account.gender[Object.keys(gender)[0] as "male" | "female"]).to.not.be
        .undefined;
      expect(account.class).to.equal(null);
      expect(toNumber(account.experience)).to.equal(0);

      expect(toNumber(account.stats.might)).to.equal(civilization.stats.might);
      expect(toNumber(account.stats.speed)).to.equal(civilization.stats.speed);
      expect(toNumber(account.stats.intellect)).to.equal(civilization.stats.intellect);

      expect(toNumber(account.attributes.atk)).to.equal(civilization.attributes.atk);
      expect(toNumber(account.attributes.def)).to.equal(civilization.attributes.def);
      expect(toNumber(account.attributes.magAtk)).to.equal(civilization.attributes.magAtk);
      expect(toNumber(account.attributes.magDef)).to.equal(civilization.attributes.magDef);
      expect(toNumber(account.attributes.range)).to.equal(civilization.attributes.range);
      expect(toNumber(account.attributes.rate)).to.equal(civilization.attributes.rate);

      expect(toNumber(account.energy)).to.equal(BASE_ENERGY);
      expect(toNumber(account.lastEnergyRefill)).to.equal(toNumber(account.createdAt));
    });
  });
});
