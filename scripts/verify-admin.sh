#!/usr/bin/env bash
set -euo pipefail
CONTRACT_ID="${1:?contract id}"
stellar contract invoke --id "$CONTRACT_ID" --network testnet --source substrata-admin -- get_admin
