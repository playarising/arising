import * as anchor from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction } from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
  createInitializeMintInstruction,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
  createMintToInstruction,
} from "@solana/spl-token";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import {
  createMetadataAccountV3,
  createMasterEditionV3,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  fromWeb3JsKeypair,
  fromWeb3JsPublicKey,
} from "@metaplex-foundation/umi-web3js-adapters";
import { createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { CollectionSetup } from "../constants";
import { findMasterEditionPda, findMetadataPda } from "./pdas";

export async function createCollectionNft(
  provider: anchor.AnchorProvider
): Promise<CollectionSetup> {
  const wallet = provider.wallet as anchor.Wallet & { payer: Keypair };
  const connection = provider.connection;

  const mint = Keypair.generate();
  const metadata = findMetadataPda(mint.publicKey);
  const edition = findMasterEditionPda(mint.publicKey);
  const associatedToken = getAssociatedTokenAddressSync(mint.publicKey, wallet.publicKey);
  const lamports = await connection.getMinimumBalanceForRentExemption(MINT_SIZE);

  const tx = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: mint.publicKey,
      lamports,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      mint.publicKey,
      0,
      wallet.publicKey,
      wallet.publicKey
    ),
    createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      associatedToken,
      wallet.publicKey,
      mint.publicKey
    ),
    createMintToInstruction(mint.publicKey, associatedToken, wallet.publicKey, 1)
  );

  await provider.sendAndConfirm(tx, [mint]);

  const umi = createUmi(connection.rpcEndpoint).use(mplTokenMetadata());
  const mintKeypair = fromWeb3JsKeypair(mint);
  const walletKeypair = fromWeb3JsKeypair(wallet.payer);
  const mintSigner = createSignerFromKeypair(umi, mintKeypair);
  const walletSigner = createSignerFromKeypair(umi, walletKeypair);

  const mintPubkey = fromWeb3JsPublicKey(mint.publicKey);
  const walletPubkey = fromWeb3JsPublicKey(wallet.publicKey);

  umi.use(signerIdentity(walletSigner, true));

  await createMetadataAccountV3(umi, {
    mint: mintPubkey,
    mintAuthority: mintSigner,
    updateAuthority: walletPubkey,
    payer: walletSigner,
    data: {
      name: "Arising Collection",
      symbol: "ARISE",
      uri: "https://mints.playarising.com/collection.json",
      sellerFeeBasisPoints: 0,
      creators: [
        {
          address: walletPubkey,
          verified: true,
          share: 100,
        },
      ],
      collection: null,
      uses: null,
    },
    isMutable: false,
    collectionDetails: { __kind: "V1", size: 0 },
  }).sendAndConfirm(umi);

  await createMasterEditionV3(umi, {
    mint: mintPubkey,
    updateAuthority: walletSigner,
    payer: walletSigner,
    mintAuthority: mintSigner,
    maxSupply: null,
  }).sendAndConfirm(umi);

  return { mint: mint.publicKey, metadata, edition };
}
