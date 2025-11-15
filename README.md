# Arising

Arising is an Anchor-based Solana program that powers a role-playing experience with quests, crafting, forging, and equipment management. The repository also contains an extensive LiteSVM-driven integration test suite.

## Prerequisites

- Rust toolchain specified in `rust-toolchain.toml` (install via `rustup toolchain install` if needed)
- Anchor CLI (`cargo install --git https://github.com/coral-xyz/anchor anchor-cli`)
- Solana toolchain (for the `solana` CLI used by Anchor)

Ensure the Cargo binaries directory is on your `PATH` so `anchor`, `cargo`, and `solana` are available.

## Building the Program

```bash
anchor build
```

This runs `cargo build-bpf` behind the scenes and emits the program shared object to `target/deploy/arising.so`, which is also loaded by the LiteSVM tests.

## Running Tests

All unit and integration tests (including LiteSVM flows) can be executed with:

```bash
cargo test -p arising -- --nocapture
```

This command covers:

- Rust unit tests under `programs/arising/src`
- LiteSVM integration suites in `programs/arising/tests`

The `--nocapture` flag prints LiteSVM logs directly if a test fails.

## Deploying to Devnet/Mainnet

The TypeScript helper `migrations/deploy-script.ts` automates the one-time
`initialize` call on public clusters,
provisioning the collection mint, metadata PDAs, and the program-owned USDC
treasury ATA. The script enforces the real USDC mint for the selected cluster:

- Devnet USDC mint: `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`
- Mainnet USDC mint: `EPjFWdd5AufqSSqeM2qCAqsD5cBA2kp4iAd4tkDWTLCh`

Run the deployer after `anchor build` with a wallet that should become the mint
authority:

```bash
# Devnet
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com \
ANCHOR_WALLET=~/.config/solana/id.json \
anchor run deploy-devnet

# Mainnet
ANCHOR_PROVIDER_URL=https://api.mainnet-beta.solana.com \
ANCHOR_WALLET=/path/to/mint-authority.json \
anchor run deploy-mainnet
```

If you prefer `anchor deploy`, set `ARISING_DEPLOY_CLUSTER=devnet|mainnet` (or
use a matching RPC endpoint) so `migrations/deploy.ts` can pick the right
USDC mint automatically.

## Energy Refill Passes

Players can buy "Energy Passes" on-chain by paying 2 USDC into the program's
USDC treasury PDA. The treasury only accepts transfers to the program-owned ATA
and the funds can later be withdrawn by the mint authority. Use the native USDC
mint for the target cluster and `initialize` will create the treasury ATA if it
does not already exist.

- Up to **5 passes** may be purchased per character every 24 hours.
- Up to **3 passes** may be redeemed per character every 24 hours.
- Redeeming a pass immediately refills the character's energy bar and bypasses
  the standard cooldown.

The LiteSVM suite (`programs/arising/tests/litesvm_energy_pass.rs`) covers
purchase, usage, daily limits, and admin withdrawals. Use it as a blueprint for
frontend flows that need to transfer USDC and call the new `buyEnergyPass` and
`redeemEnergyPass` instructions.

## Repository Layout

- `programs/arising/src/` – Anchor program logic (quests, recipes, equipment, helpers)
- `programs/arising/tests/` – LiteSVM-based integration tests and helper modules
- `patches/` – Local patches (e.g., Solana account info shim)
- `Anchor.toml` / `Cargo.toml` – workspace configuration

## Tips

- Always run `anchor build` before running LiteSVM tests so the latest `.so` is available.
- If you add new resources or equipment types, update the codex helpers and the recipe/quest tests accordingly.
