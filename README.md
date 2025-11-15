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

## Gated Minting

Character minting is protected by a gate authority so only the website (or an
approved backend) can trigger `mintCharacter`. During `initialize` the
`mint_state` PDA records the authority public key. Every call to
`mintCharacter` must now include a `mintAuthority` signer that matches this
stored key. The signer does not pay for the transaction—it simply approves the
mint alongside the user’s wallet.

A minimal TypeScript example lives at `tests/gated-mint.test.ts`. Run it with
`ts-node`/`bun` after executing the `initialize` instruction (or on a cluster
where the collection PDAs already exist). The script demonstrates:

1. Deriving the required PDAs for the mint state, character PDA, and metadata
   accounts.
2. Showing that a mint without the gate authority signature fails with the
   `Unauthorized` error.
3. Including the website signer when invoking `mintCharacter`.

Use that script as a reference for wiring the website flow so every mint goes
through the authenticated backend.

## Energy Refill Passes

Players can buy "Energy Passes" on-chain by paying 2 USDC into the program's
USDC treasury PDA. The treasury only accepts transfers to the program-owned ATA
and the funds can later be withdrawn by the mint authority.

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
