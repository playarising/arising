#!/usr/bin/env bash
set -euo pipefail

echo "Starting solana-test-validator with token_metadata clone..."
solana-test-validator \
  --reset \
  --clone-upgradeable-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s \
  --url https://api.mainnet-beta.solana.com \
  --bind-address 127.0.0.1 &
VALIDATOR_PID=$!

cleanup() {
  echo "Stopping solana-test-validator..."
  kill "${VALIDATOR_PID}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

# Give the validator a moment to boot.
sleep 2

echo "Running anchor tests using external validator..."
ANCHOR_PROVIDER_URL="http://127.0.0.1:8899" anchor test --skip-local-validator "$@"
