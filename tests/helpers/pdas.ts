import { PublicKey } from "@solana/web3.js";
import { TOKEN_METADATA_PROGRAM_ID } from "../constants";

const CHARACTER_SEED = Buffer.from("character");
const CHARACTER_MINT_STATE_SEED = Buffer.from("character_mint_state");
const CHARACTER_NFT_MINT_SEED = Buffer.from("character_nft_mint");
const CHARACTER_NFT_TOKEN_SEED = Buffer.from("character_nft_token");
const METADATA_SEED = Buffer.from("metadata");
const MASTER_EDITION_SEED = Buffer.from("edition");
const COLLECTION_MINT_SEED = Buffer.from("collection_mint");
const COLLECTION_TOKEN_SEED = Buffer.from("collection_token");
const RESOURCE_MINT_SEED = Buffer.from("resource_mint");
const RESOURCE_SEEDS = {
  gold: Buffer.from("gold"),
  log: Buffer.from("log"),
  plank: Buffer.from("plank"),
  woodenSword: Buffer.from("wooden_sword"),
} as const;

export type ResourceMintKey = keyof typeof RESOURCE_SEEDS;

export const deriveMintStatePda = (programId: PublicKey): PublicKey => {
  return PublicKey.findProgramAddressSync([CHARACTER_MINT_STATE_SEED], programId)[0];
};

export const deriveCollectionMintPda = (programId: PublicKey): PublicKey => {
  return PublicKey.findProgramAddressSync([COLLECTION_MINT_SEED], programId)[0];
};

export const deriveCollectionTokenPda = (programId: PublicKey): PublicKey => {
  return PublicKey.findProgramAddressSync([COLLECTION_TOKEN_SEED], programId)[0];
};

export const deriveCharacterPda = (
  civilizationIndex: number,
  characterId: number,
  programId: PublicKey
): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [
      CHARACTER_SEED,
      Buffer.from([civilizationIndex]),
      new Uint8Array(new Uint16Array([characterId]).buffer),
    ],
    programId
  )[0];
};

export const deriveCharacterMintPda = (
  civilizationIndex: number,
  characterId: number,
  programId: PublicKey
): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [
      CHARACTER_NFT_MINT_SEED,
      Buffer.from([civilizationIndex]),
      new Uint8Array(new Uint16Array([characterId]).buffer),
    ],
    programId
  )[0];
};

export const deriveCharacterTokenPda = (
  civilizationIndex: number,
  characterId: number,
  programId: PublicKey
): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [
      CHARACTER_NFT_TOKEN_SEED,
      Buffer.from([civilizationIndex]),
      new Uint8Array(new Uint16Array([characterId]).buffer),
    ],
    programId
  )[0];
};

export const findMetadataPda = (
  mint: PublicKey,
  programId: PublicKey = TOKEN_METADATA_PROGRAM_ID
): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [METADATA_SEED, programId.toBuffer(), mint.toBuffer()],
    programId
  )[0];
};

export const findMasterEditionPda = (
  mint: PublicKey,
  programId: PublicKey = TOKEN_METADATA_PROGRAM_ID
): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [METADATA_SEED, programId.toBuffer(), mint.toBuffer(), MASTER_EDITION_SEED],
    programId
  )[0];
};

export const deriveResourceMintPda = (
  resource: ResourceMintKey,
  programId: PublicKey
): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [RESOURCE_MINT_SEED, RESOURCE_SEEDS[resource]],
    programId
  )[0];
};
