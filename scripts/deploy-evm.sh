#!/bin/bash
set -euo pipefail

CHAIN="${1:?usage: deploy-evm.sh <ethereum|base|arbitrum>}"

case "$CHAIN" in
  ethereum) RPC=$ETH_RPC_URL ;;
  base)     RPC=$BASE_RPC_URL ;;
  arbitrum) RPC=$ARB_RPC_URL ;;
  *) echo "unknown chain: $CHAIN"; exit 1 ;;
esac

cd evm

echo "Building..."
forge build

echo "Deploying to $CHAIN..."
forge script script/Deploy.s.sol \
  --rpc-url "$RPC" \
  --broadcast \
  --verify \
  -vvvv

echo "Done."
