#!/usr/bin/env bun
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

/**
 * Program ID resolution:
 * - Allow overriding via env (useful for deploying to a freshly created program address)
 * - Fall back to the IDL address (kept in sync with Anchor.toml / declare_id!)
 */
const PROGRAM_ID = new PublicKey(
  process.env.ARISING_PROGRAM_ID ?? idlJson.address,
);

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

// Ensure the IDL address matches any override provided via env and patch missing types.
const idl: ArisingIdl = idlJson as ArisingIdl;
(idl as any).address = PROGRAM_ID.toBase58();

// Replace any missing defined types with a scalar `u16` so the Borsh coder can lay them out.
const collectDefinedNames = (obj: any, acc: Set<string>) => {
  if (Array.isArray(obj)) {
    obj.forEach((v) => collectDefinedNames(v, acc));
    return;
  }
  if (obj && typeof obj === "object") {
    const anyObj = obj as any;
    if (anyObj.defined && typeof anyObj.defined.name === "string") {
      acc.add(anyObj.defined.name);
    }
    Object.values(anyObj).forEach((v) => collectDefinedNames(v, acc));
  }
};
const existingTypes = new Set<string>(
  ((idl as any).types as any[] | []).map((t: any) => t.name),
);
const referenced = new Set<string>();
collectDefinedNames(idl, referenced);
const missing = new Set(
  [...referenced].filter((name) => !existingTypes.has(name)),
);

const replaceMissingDefined = (obj: any) => {
  if (Array.isArray(obj)) {
    obj.forEach(replaceMissingDefined);
    return;
  }
  if (obj && typeof obj === "object") {
    const anyObj = obj as any;
    if (anyObj.defined && missing.has(anyObj.defined.name)) {
      // Replace the entire object with the scalar type
      // Caller holds the reference, so mutate in place on parent iteration.
      Object.assign(anyObj, { type: "u16" });
      delete anyObj.defined;
      return;
    }
    Object.entries(anyObj).forEach(([k, v]) => {
      if (v && typeof v === "object") {
        const anyV = v as any;
        if (anyV.defined && missing.has(anyV.defined.name)) {
          anyObj[k] = "u16";
        } else {
          replaceMissingDefined(v);
        }
      }
    });
  }
};
replaceMissingDefined(idl);

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

async function ensureProgramIsDeployed(
  provider: anchor.AnchorProvider,
  programId: PublicKey,
): Promise<void> {
  const accountInfo = await provider.connection.getAccountInfo(programId);
  if (!accountInfo || !accountInfo.executable) {
    throw new Error(
      `Program ${programId.toBase58()} is not deployed to ${provider.connection.rpcEndpoint}. ` +
      "Deploy the program (e.g. `anchor deploy --provider.cluster devnet`) or set ARISING_PROGRAM_ID to the deployed address.",
    );
  }
}

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
  // Always use the patched IDL to avoid missing-type decode errors.
  return new anchor.Program<ArisingIdl>(idl, provider);
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
  await ensureProgramIsDeployed(provider, program.programId);

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
    .rpc({ skipPreflight: true });

  console.log(
    `[${config.label}] Initialize complete. Treasury ATA: ${usdcTreasury.toBase58()}`,
  );
}

async function main() {
  const endpoint = process.env.ANCHOR_PROVIDER_URL;
  if (!endpoint) {
    throw new Error("ANCHOR_PROVIDER_URL must be set (e.g. https://api.devnet.solana.com)");
  }

  const cluster =
    toClusterKey(process.argv[2]) ?? detectClusterFromEndpoint(endpoint);
  if (!cluster) {
    throw new Error(
      "Unable to determine cluster. Pass 'devnet' or 'mainnet' as an arg, or set ANCHOR_PROVIDER_URL accordingly.",
    );
  }

  const walletPath = process.env.ANCHOR_WALLET;
  if (!walletPath) {
    throw new Error("ANCHOR_WALLET must be set to your payer keypair path");
  }

  const wallet = new anchor.Wallet(
    anchor.web3.Keypair.fromSecretKey(
      Uint8Array.from(
        JSON.parse(require("fs").readFileSync(walletPath, "utf8")),
      ),
    ),
  );
  const connection = new anchor.web3.Connection(endpoint, "confirmed");
  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  await runDeployment(provider, cluster);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
