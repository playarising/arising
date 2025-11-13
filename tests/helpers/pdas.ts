import { PublicKey } from "@solana/web3.js";
import { TOKEN_METADATA_PROGRAM_ID } from "../constants";

const CHARACTER_SEED = Buffer.from("character");
const CHARACTER_MINT_STATE_SEED = Buffer.from("character_mint_state");
const CHARACTER_NFT_MINT_SEED = Buffer.from("character_nft_mint");
const CHARACTER_NFT_TOKEN_SEED = Buffer.from("character_nft_token");
const METADATA_SEED = Buffer.from("metadata");
const MASTER_EDITION_SEED = Buffer.from("edition");

export const deriveMintStatePda = (programId: PublicKey): PublicKey => {
  return PublicKey.findProgramAddressSync([CHARACTER_MINT_STATE_SEED], programId)[0];
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
