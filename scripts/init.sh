#!/usr/bin/env bash
# Initialize a deployed Sorobill contract (set billing admin).
set -euo pipefail

NETWORK="testnet"
SOURCE="sorobill-admin"
CONTRACT_ID=""

usage() {
  echo "Usage: $0 --id CONTRACT_ID [--network testnet|mainnet] [--source IDENTITY]"
  exit 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --network) NETWORK="$2"; shift 2 ;;
    --source) SOURCE="$2"; shift 2 ;;
    --id) CONTRACT_ID="$2"; shift 2 ;;
    *) usage ;;
  esac
done

if [[ -z "$CONTRACT_ID" ]]; then
  usage
fi

if ! command -v stellar >/dev/null 2>&1; then
  echo "error: stellar CLI is required"
  exit 1
fi

ADMIN=$(stellar keys address "$SOURCE")

echo "Initializing contract $CONTRACT_ID"
echo "  Admin: $ADMIN"
echo "  Network: $NETWORK"

stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOURCE" \
  --network "$NETWORK" \
  -- \
  initialize \
  --admin "$ADMIN"

echo "Initialized successfully."
