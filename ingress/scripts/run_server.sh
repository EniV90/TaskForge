#!/usr/bin/env bash
# File: ingress/scripts/run_server.sh
# - The frontend must be built before embedding it, if changes are made to the frontend, there's a risk the server can be rebuilt
# - without the frontend which can cause problem, which can lead to why changes aren't showing, so this script it to automate the process

# Get script directory
SCRIPT_PATH="$(cd "$(dirname "$0")" && pwd)"

# Go to project root
cd "$SCRIPT_PATH/../.."

echo "Now in: $(pwd)"

# Go to frontend safely
cd frontend || { echo "❌ frontend folder not found"; exit 1; }

echo "Frontend dir: $(pwd)"

# Build frontend
npm install
npm run build

# Go back to ingress
cd ../ingress

cargo clean

 # Export environment variables
export $(cat .env | xargs)

# Run server
cargo run