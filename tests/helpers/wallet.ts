import * as anchor from "@coral-xyz/anchor";

const MIN_FUNDS = 5 * anchor.web3.LAMPORTS_PER_SOL;


export async function ensureWalletHasLamports(
  provider: anchor.AnchorProvider,
  wallet: anchor.Wallet
) {
  const connection = provider.connection;

  const current = await connection.getBalance(wallet.publicKey);
  if (current < MIN_FUNDS) {
    const sig = await connection.requestAirdrop(wallet.publicKey, MIN_FUNDS);
    await connection.confirmTransaction(sig, "confirmed");
  }
}