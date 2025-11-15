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

## Repository Layout

- `programs/arising/src/` – Anchor program logic (quests, recipes, equipment, helpers)
- `programs/arising/tests/` – LiteSVM-based integration tests and helper modules
- `patches/` – Local patches (e.g., Solana account info shim)
- `Anchor.toml` / `Cargo.toml` – workspace configuration

## Tips

- Always run `anchor build` before running LiteSVM tests so the latest `.so` is available.
- If you add new resources or equipment types, update the codex helpers and the recipe/quest tests accordingly.
