#!/usr/bin/env bash
# Deploy Substrata Soroban contract to a Stellar network.
set -euo pipefail

NETWORK="testnet"
SOURCE="substrata-admin"
WASM_PATH="target/wasm32-unknown-unknown/release/substrata.wasm"

usage() {
  echo "Usage: $0 [--network testnet|mainnet] [--source IDENTITY] [--wasm PATH]"
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --network) NETWORK="$2"; shift 2 ;;
    --source) SOURCE="$2"; shift 2 ;;
    --wasm) WASM_PATH="$2"; shift 2 ;;
    *) usage ;;
  esac
done

if ! command -v stellar >/dev/null 2>&1; then
  echo "error: stellar CLI is required. Install from https://developers.stellar.org/docs/tools/cli"
  exit 1
fi

if [[ ! -f "$WASM_PATH" ]]; then
  echo "Building contract wasm..."
  cargo build --target wasm32-unknown-unknown --release -p substrata
fi

echo "Deploying $WASM_PATH to $NETWORK as $SOURCE..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_PATH" \
  --source "$SOURCE" \
  --network "$NETWORK" \
  --alias substrata)

echo ""
echo "Deployed Substrata contract"
echo "  Network:     $NETWORK"
echo "  Contract ID: $CONTRACT_ID"
echo ""
echo "Next: ./scripts/init.sh --network $NETWORK --source $SOURCE --id $CONTRACT_ID"
