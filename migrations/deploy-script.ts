import * as anchor from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import idlJson from "../target/idl/arising.json";
import type { Arising } from "../target/types/arising";

type ClusterKey = "devnet" | "mainnet";

interface ClusterConfig {
  label: string;
  usdcMint: PublicKey;
}

const PROGRAM_ID = new PublicKey("FLTa6pSm3WPWNPrqVu1pCuAmjgmVJkhDvyxMcSPSAfLA");

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
);

const SEEDS = {
  mintState: Buffer.from("character_mint_state"),
  collectionMint: Buffer.from("collection_mint"),
  collectionToken: Buffer.from("collection_token"),
} as const;

const CLUSTER_CONFIG: Record<ClusterKey, ClusterConfig> = {
  devnet: {
    label: "devnet",
    usdcMint: new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"),
  },
  mainnet: {
    label: "mainnet",
    usdcMint: new PublicKey("EPjFWdd5AufqSSqeM2qCAqsD5cBA2kp4iAd4tkDWTLCh"),
  },
};

type ArisingIdl = anchor.Idl & Arising;
type ArisingProgram = anchor.Program<ArisingIdl>;

const idl: ArisingIdl = idlJson as ArisingIdl;

function metadataPda(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID,
  )[0];
}

function masterEditionPda(mint: PublicKey): PublicKey {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
      Buffer.from("edition"),
    ],
    TOKEN_METADATA_PROGRAM_ID,
  )[0];
}

type Workspace = {
  Arising?: ArisingProgram;
};

function getProgram(provider: anchor.AnchorProvider): ArisingProgram {
  const workspaceProgram = (anchor.workspace as Workspace | undefined)?.Arising;
  if (workspaceProgram) {
    return workspaceProgram;
  }
  const program: ArisingProgram = new anchor.Program<ArisingIdl>(
    idl,
    PROGRAM_ID,
    provider,
  );
  return program;
}

type InitializeAccounts = {
  mintState: PublicKey;
  authority: PublicKey;
  collectionMint: PublicKey;
  collectionTokenAccount: PublicKey;
  collectionMetadata: PublicKey;
  collectionMasterEdition: PublicKey;
  tokenMetadataProgram: PublicKey;
  tokenProgram: PublicKey;
  usdcMint: PublicKey;
  usdcTreasury: PublicKey;
  systemProgram: PublicKey;
  associatedTokenProgram: PublicKey;
  rent: PublicKey;
};

export function toClusterKey(input?: string | null): ClusterKey | undefined {
  if (!input) {
    return undefined;
  }
  const normalized = input.toLowerCase();
  if (normalized.startsWith("dev")) {
    return "devnet";
  }
  if (normalized.startsWith("main")) {
    return "mainnet";
  }
  return undefined;
}

export function detectClusterFromEndpoint(
  endpoint?: string | null,
): ClusterKey | undefined {
  if (!endpoint) {
    return undefined;
  }
  if (endpoint.includes("devnet")) {
    return "devnet";
  }
  if (endpoint.includes("mainnet")) {
    return "mainnet";
  }
  return undefined;
}

export async function runDeployment(
  provider: anchor.AnchorProvider,
  cluster: ClusterKey,
): Promise<void> {
  const config = CLUSTER_CONFIG[cluster];
  if (!config) {
    throw new Error(`Unknown cluster "${cluster}"`);
  }

  const program = getProgram(provider);
  const [mintState] = PublicKey.findProgramAddressSync(
    [SEEDS.mintState],
    program.programId,
  );

  const existing = await provider.connection.getAccountInfo(mintState);
  if (existing) {
    console.log(
      `[${config.label}] Mint state already exists at ${mintState.toBase58()}, skipping initialize.`,
    );
    return;
  }

  const [collectionMint] = PublicKey.findProgramAddressSync(
    [SEEDS.collectionMint],
    program.programId,
  );
  const [collectionTokenAccount] = PublicKey.findProgramAddressSync(
    [SEEDS.collectionToken],
    program.programId,
  );
  const collectionMetadata = metadataPda(collectionMint);
  const collectionMasterEdition = masterEditionPda(collectionMint);
  const usdcTreasury = getAssociatedTokenAddressSync(
    config.usdcMint,
    mintState,
    true,
  );

  console.log(
    `[${config.label}] Initializing mint state ${mintState.toBase58()} with USDC mint ${config.usdcMint.toBase58()}`,
  );

  const initializeAccounts: InitializeAccounts = {
    mintState,
    authority: provider.wallet.publicKey,
    collectionMint,
    collectionTokenAccount,
    collectionMetadata,
    collectionMasterEdition,
    tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    tokenProgram: TOKEN_PROGRAM_ID,
    usdcMint: config.usdcMint,
    usdcTreasury,
    systemProgram: SystemProgram.programId,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  };

  await program.methods
    .initialize()
    .accounts(initializeAccounts)
    .rpc();

  console.log(
    `[${config.label}] Initialize complete. Treasury ATA: ${usdcTreasury.toBase58()}`,
  );
}

export async function deployWithAutomaticCluster(
  provider: anchor.AnchorProvider,
): Promise<void> {
  const explicit = toClusterKey(process.env.ARISING_DEPLOY_CLUSTER ?? null);
  const inferred = detectClusterFromEndpoint(
    provider.connection.rpcEndpoint ?? "",
  );
  const cluster = explicit ?? inferred;

  if (!cluster) {
    console.log(
      "[deploy] No devnet/mainnet cluster detected. Set ARISING_DEPLOY_CLUSTER to run the initializer.",
    );
    return;
  }

  await runDeployment(provider, cluster);
}

if (require.main === module) {
  (async () => {
    const cluster = toClusterKey(process.argv[2]);
    if (!cluster) {
      console.error("Usage: ts-node migrations/deploy-script.ts <devnet|mainnet>");
      process.exit(1);
    }
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    await runDeployment(provider, cluster);
  })().catch((err: unknown) => {
    console.error(err);
    process.exit(1);
  });
}
