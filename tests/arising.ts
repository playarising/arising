import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Arising } from "../target/types/arising.ts";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createInitializeMintInstruction,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAccount,
  getMint,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import {
  createCreateMetadataAccountV3Instruction,
  createCreateMasterEditionV3Instruction,
} from "@metaplex-foundation/mpl-token-metadata";
import { Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

const CIVILIZATIONS = [
  {
    key: "ard",
    variant: { ard: {} },
    stats: { might: 3, speed: 2, intellect: 1 },
    attributes: { atk: 2, def: 1, magAtk: 0, magDef: 0, range: 0, rate: 0 },
  },
  {
    key: "hartenn",
    variant: { hartenn: {} },
    stats: { might: 3, speed: 1, intellect: 2 },
    attributes: { atk: 1, def: 0, magAtk: 0, magDef: 2, range: 0, rate: 0 },
  },
  {
    key: "ikarans",
    variant: { ikarans: {} },
    stats: { might: 1, speed: 3, intellect: 2 },
    attributes: { atk: 1, def: 0, magAtk: 0, magDef: 0, range: 1, rate: 1 },
  },
  {
    key: "zhand",
    variant: { zhand: {} },
    stats: { might: 1, speed: 2, intellect: 3 },
    attributes: { atk: 0, def: 0, magAtk: 2, magDef: 0, range: 0, rate: 1 },
  },
  {
    key: "shinkari",
    variant: { shinkari: {} },
    stats: { might: 2, speed: 3, intellect: 1 },
    attributes: { atk: 1, def: 0, magAtk: 1, magDef: 0, range: 0, rate: 1 },
  },
  {
    key: "tarki",
    variant: { tarki: {} },
    stats: { might: 3, speed: 2, intellect: 1 },
    attributes: { atk: 2, def: 0, magAtk: 0, magDef: 0, range: 0, rate: 1 },
  },
] as const;

const CIVILIZATION_ORDER = CIVILIZATIONS.map((c) => c.key);

type CollectionSetup = {
  mint: PublicKey;
  metadata: PublicKey;
  edition: PublicKey;
};

describe("arising", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.arising as Program<Arising>;
  const wallet = provider.wallet as anchor.Wallet & { payer: anchor.web3.Keypair };

  const [mintStatePda] = PublicKey.findProgramAddressSync(
    [Buffer.from("character_mint_state")],
    program.programId
  );

  let collection: CollectionSetup;

  before(async () => {
    collection = await createCollectionNft(provider);
    await program.methods
      .initialize()
      .accounts({
        mintState: mintStatePda,
        authority: wallet.publicKey,
        collectionMint: collection.mint,
        collectionMetadata: collection.metadata,
        collectionMasterEdition: collection.edition,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
  });

  it("mints civilization NFTs, updates caps, and emits metadata", async () => {
    for (const [index, civilization] of CIVILIZATIONS.entries()) {
      const characterId = 1; // first mint per civilization
      const civIndex = civilizationIndex(civilization.key);

      const characterPda = deriveCharacterPda(civIndex, characterId, program.programId);
      const characterMintPda = deriveCharacterMintPda(
        civIndex,
        characterId,
        program.programId
      );
      const characterTokenPda = deriveCharacterTokenPda(
        civIndex,
        characterId,
        program.programId
      );
      const metadataPda = findMetadataPda(characterMintPda);
      const masterEditionPda = findMasterEditionPda(characterMintPda);

      await program.methods
        .mintCharacter(
          civilization.variant,
          characterId,
          index % 2 === 0 ? { male: {} } : { female: {} }
        )
        .accounts({
          character: characterPda,
          authority: wallet.publicKey,
          mintState: mintStatePda,
          characterMint: characterMintPda,
          characterTokenAccount: characterTokenPda,
          metadata: metadataPda,
          masterEdition: masterEditionPda,
          collectionMint: collection.mint,
          collectionMetadata: collection.metadata,
          collectionMasterEdition: collection.edition,
          tokenProgram: TOKEN_PROGRAM_ID,
          tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      const account = await program.account.character.fetch(characterPda);
      expect(account.civilizationCharacterId).to.equal(characterId);
      expect(account.civilization).to.have.property(civilization.key);
      expect(account.gender).to.have.property(index % 2 === 0 ? "male" : "female");
      expect(account.stats.might).to.equal(civilization.stats.might);
      expect(account.stats.speed).to.equal(civilization.stats.speed);
      expect(account.stats.intellect).to.equal(civilization.stats.intellect);
      expect(account.attributes.atk).to.equal(civilization.attributes.atk);
      expect(account.attributes.def).to.equal(civilization.attributes.def);
      expect(account.attributes.magAtk).to.equal(civilization.attributes.magAtk);
      expect(account.attributes.magDef).to.equal(civilization.attributes.magDef);
      expect(account.attributes.range).to.equal(civilization.attributes.range);
      expect(account.attributes.rate).to.equal(civilization.attributes.rate);
      expect(account.nftMint.toBase58()).to.equal(characterMintPda.toBase58());
      expect(account.nftMetadata.toBase58()).to.equal(metadataPda.toBase58());
      expect(account.nftMasterEdition.toBase58()).to.equal(masterEditionPda.toBase58());
      expect(account.nftTokenAccount.toBase58()).to.equal(
        characterTokenPda.toBase58()
      );

      const mintInfo = await getMint(provider.connection, characterMintPda);
      expect(Number(mintInfo.supply)).to.equal(1);
      expect(mintInfo.decimals).to.equal(0);

      const tokenAccount = await getAccount(provider.connection, characterTokenPda);
      expect(Number(tokenAccount.amount)).to.equal(1);
      expect(tokenAccount.owner.toBase58()).to.equal(wallet.publicKey.toBase58());

      await program.methods
        .emitCharacterMetadata(civilization.variant, characterId)
        .accounts({ character: characterPda })
        .rpc();
    }

    // Increase Ard cap to ensure admin control works
    await program.methods
      .updateCivilizationCap(CIVILIZATIONS[0].variant, 1200)
      .accounts({
        mintState: mintStatePda,
        authority: wallet.publicKey,
      })
      .rpc();
  });
});

function civilizationIndex(key: string): number {
  const idx = CIVILIZATION_ORDER.indexOf(key);
  if (idx === -1) {
    throw new Error(`Unknown civilization ${key}`);
  }
  return idx;
}

function deriveCharacterPda(
  civilizationIndex: number,
  characterId: number,
  programId: PublicKey
): PublicKey {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("character"),
      Buffer.from([civilizationIndex]),
      toCharacterIdBuffer(characterId),
    ],
    programId
  )[0];
}

function deriveCharacterMintPda(
  civilizationIndex: number,
  characterId: number,
  programId: PublicKey
): PublicKey {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("character_nft_mint"),
      Buffer.from([civilizationIndex]),
      toCharacterIdBuffer(characterId),
    ],
    programId
  )[0];
}

function deriveCharacterTokenPda(
  civilizationIndex: number,
  characterId: number,
  programId: PublicKey
): PublicKey {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("character_nft_token"),
      Buffer.from([civilizationIndex]),
      toCharacterIdBuffer(characterId),
    ],
    programId
  )[0];
}

function toCharacterIdBuffer(characterId: number): Buffer {
  const buf = Buffer.alloc(2);
  buf.writeUInt16LE(characterId);
  return buf;
}

function findMetadataPda(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID
  )[0];
}

function findMasterEditionPda(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      Buffer.from("edition"),
    ],
    TOKEN_METADATA_PROGRAM_ID
  )[0];
}

async function createCollectionNft(
  provider: anchor.AnchorProvider
): Promise<CollectionSetup> {
  const wallet = provider.wallet as anchor.Wallet & { payer: Keypair };
  const connection = provider.connection;
  const mint = Keypair.generate();
  const metadata = findMetadataPda(mint.publicKey);
  const edition = findMasterEditionPda(mint.publicKey);
  const associatedToken = getAssociatedTokenAddressSync(
    mint.publicKey,
    wallet.publicKey
  );
  const lamports = await connection.getMinimumBalanceForRentExemption(MINT_SIZE);

  const metadataArgs = {
    data: {
      name: "Arising Collection",
      symbol: "ARISE",
      uri: "https://mints.playarising.com/collection.json",
      sellerFeeBasisPoints: 0,
      creators: [
        {
          address: wallet.publicKey,
          verified: true,
          share: 100,
        },
      ],
      collection: null,
      uses: null,
    },
    isMutable: false,
    collectionDetails: { __kind: "V1", size: new anchor.BN(0) },
  };

  const tx = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: mint.publicKey,
      lamports,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(mint.publicKey, 0, wallet.publicKey, wallet.publicKey),
    createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      associatedToken,
      wallet.publicKey,
      mint.publicKey
    ),
    createMintToInstruction(mint.publicKey, associatedToken, wallet.publicKey, 1),
    createCreateMetadataAccountV3Instruction(
      {
        metadata,
        mint: mint.publicKey,
        mintAuthority: wallet.publicKey,
        payer: wallet.publicKey,
        updateAuthority: wallet.publicKey,
      },
      { createMetadataAccountArgsV3: metadataArgs }
    ),
    createCreateMasterEditionV3Instruction(
      {
        edition,
        mint: mint.publicKey,
        updateAuthority: wallet.publicKey,
        mintAuthority: wallet.publicKey,
        payer: wallet.publicKey,
        metadata,
      },
      { createMasterEditionArgs: { maxSupply: null } }
    )
  );

  await provider.sendAndConfirm(tx, [mint]);

  return { mint: mint.publicKey, metadata, edition };
}
