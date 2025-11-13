import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Arising } from "../target/types/arising";

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

type CnftAccounts = {
  treeAuthority: anchor.web3.PublicKey;
  merkleTree: anchor.web3.PublicKey;
  treeDelegate: anchor.web3.PublicKey;
  collectionAuthority: anchor.web3.PublicKey;
  collectionAuthorityRecordPda: anchor.web3.PublicKey;
  collectionMint: anchor.web3.PublicKey;
  collectionMetadata: anchor.web3.PublicKey;
  collectionEdition: anchor.web3.PublicKey;
  bubblegumSigner: anchor.web3.PublicKey;
  bubblegumProgram: anchor.web3.PublicKey;
  compressionProgram: anchor.web3.PublicKey;
  logWrapper: anchor.web3.PublicKey;
  tokenMetadataProgram: anchor.web3.PublicKey;
};

describe("arising", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.arising as Program<Arising>;
  const wallet = provider.wallet as anchor.Wallet & { payer: anchor.web3.Keypair };

  let cnftAccounts: CnftAccounts;
  const [mintStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("character_mint_state")],
    program.programId
  );

  before(async () => {
    cnftAccounts = await createCnftAccounts(provider);
    await program.methods
      .initialize()
      .accounts({
        mintState: mintStatePda,
        authority: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });

  it("mints a character for every civilization with correct defaults", async () => {
    for (const [index, civilization] of CIVILIZATIONS.entries()) {
      const characterId = new anchor.BN(index + 1);
      const [characterPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("character"),
          wallet.publicKey.toBuffer(),
          characterId.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );

      await program.methods
        .mintCharacter(
          characterId,
          index % 2 === 0 ? { male: {} } : { female: {} },
          civilization.variant
        )
        .accounts({
          character: characterPda,
          authority: wallet.publicKey,
          mintState: mintStatePda,
          cnft: cnftAccounts,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const account = await program.account.character.fetch(characterPda);

      expect(account.experience.toNumber()).to.equal(0);
      expect(account.gender).to.have.property(index % 2 === 0 ? "male" : "female");
      expect(account.civilization).to.have.property(civilization.key);

      expect(account.stats.might).to.equal(civilization.stats.might);
      expect(account.stats.speed).to.equal(civilization.stats.speed);
      expect(account.stats.intellect).to.equal(civilization.stats.intellect);

      expect(account.attributes.atk).to.equal(civilization.attributes.atk);
      expect(account.attributes.def).to.equal(civilization.attributes.def);
      expect(account.attributes.magAtk).to.equal(civilization.attributes.magAtk);
      expect(account.attributes.magDef).to.equal(civilization.attributes.magDef);
      expect(account.attributes.range).to.equal(civilization.attributes.range);
      expect(account.attributes.rate).to.equal(civilization.attributes.rate);

      expect(account.cnftTree.toBase58()).to.equal(cnftAccounts.merkleTree.toBase58());
      expect(account.cnftLeafIndex.toNumber()).to.equal(0);
      expect(account.cnftHash).to.have.lengthOf(32);
    }
  });
});

async function createCnftAccounts(provider: anchor.AnchorProvider): Promise<CnftAccounts> {
  const createAccount = async (space = 0) => {
    const keypair = anchor.web3.Keypair.generate();
    const lamports =
      (await provider.connection.getMinimumBalanceForRentExemption(space)) + 1_000_000;

    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: provider.wallet.publicKey,
        newAccountPubkey: keypair.publicKey,
        lamports,
        space,
        programId: anchor.web3.SystemProgram.programId,
      })
    );

    await provider.sendAndConfirm(tx, [keypair]);
    return keypair.publicKey;
  };

  return {
    treeAuthority: await createAccount(),
    merkleTree: await createAccount(1024),
    treeDelegate: await createAccount(),
    collectionAuthority: await createAccount(),
    collectionAuthorityRecordPda: await createAccount(),
    collectionMint: await createAccount(),
    collectionMetadata: await createAccount(256),
    collectionEdition: await createAccount(256),
    bubblegumSigner: await createAccount(),
    bubblegumProgram: await createAccount(),
    compressionProgram: await createAccount(),
    logWrapper: await createAccount(),
    tokenMetadataProgram: await createAccount(),
  };
}
