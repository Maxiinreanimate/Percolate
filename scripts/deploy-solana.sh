#!/bin/bash
set -euo pipefail

CLUSTER="${1:-devnet}"

echo "Building Anchor program..."
cd solana
anchor build

echo "Deploying to $CLUSTER..."
anchor deploy --provider.cluster "$CLUSTER"

echo "Done. Program ID:"
solana address -k target/deploy/percolate-keypair.json
