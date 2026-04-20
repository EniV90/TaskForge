# SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
# cd $SCRIPTPATH
# cd ../../

# # to-do/scripts/test_server_com.sh
# docker-compose build
# docker compose up -d
# sleep 1
# export $(cat ./to-do/.env | xargs)

# cargo build \
# --manifest-path ./to-do/networking/Cargo.toml \
# --features auth-http \
# --release \
# --no-default-features
# cargo build \
# --release

# cargo run \
# --manifest-path ./to-do/networking/Cargo.toml \
# --features auth-http \
# --release 
# --no-default-features &
# TO_DO_PID=$!
# cargo run \
# --release &
# --manifest-path ./auth/networking/Cargo.toml \
# AUTH_PID=$!

# sleep 2
# curl -X POST http://127.0.0.1:8081/api/v1/users/create \
# -H "Content-Type: application/json" \
# -d '{
# "email": "test@gmail.com",
# "password": "password"
# }'

# token=$(curl \
# -u test@gmail.com:password \
# -X GET http://127.0.0.1:8081/api/v1/auth/login)
# token=$(echo "$token" | tr -d '\r\n' | sed 's/^"//' | sed 's/"$//')

# response=$(curl -X POST http://127.0.0.1:8080/api/v1/create \
# -H "Content-Type: application/json" \
# -H "token: $token" \
# -d '{
# "title": "code",
# "status": "PENDING"
# }')
# sleep 1
# echo $response
# sleep 2

# kill $TO_DO_PID
# kill $AUTH_PID
# docker compose down

# SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
# cd $SCRIPTPATH
# cd ../../

# docker-compose build
# docker compose up -d
# sleep 1

# export $(cat ./to-do/.env | xargs)

# # Build
# cargo build \
#   --manifest-path ./to-do/networking/Cargo.toml \
#   --features auth-http \
#   --release \
#   --no-default-features

# cargo build --release

# # Run TO-DO service (8080)
# cargo run \
#   --manifest-path ./to-do/networking/Cargo.toml \
#   --features auth-http \
#   --release \
#   --no-default-features &
# TO_DO_PID=$!

# # Run AUTH service (8081)
# cargo run \
#   --manifest-path ../auth/networking/Cargo.toml \
#   --release &
# AUTH_PID=$!

# # Wait for servers (IMPORTANT)
# sleep 3

# # Create user
# curl -X POST http://127.0.0.1:8081/api/v1/users/create \
#   -H "Content-Type: application/json" \
#   -d '{
#     "email": "test2@gmail.com",
#     "password": "password"
#   }'

# # Get token
# token=$(curl \
#   -u test@gmail.com:password \
#   -X GET http://127.0.0.1:8081/api/v1/auth/login)

# token=$(echo "$token" | tr -d '\r\n' | sed 's/^"//' | sed 's/"$//')

# echo "TOKEN: $token"

# # Create todo
# response=$(curl -X POST http://127.0.0.1:8080/api/v1/create \
#   -H "Content-Type: application/json" \
#   -H "token: $token" \
#   -d '{
#     "title": "code",
#     "status": "PENDING"
#   }')

# echo "$response"

# sleep 2

# kill $TO_DO_PID
# kill $AUTH_PID

# docker compose down

#!/usr/bin/env bash

set -e

# Get script directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
ROOT="$SCRIPTPATH/../.."

cd "$ROOT"

echo "Running from: $(pwd)"

# Start docker (Postgres)
docker compose down -v || true
docker compose up -d

# Load env
export $(cat ./to-do/.env | xargs)

# Build everything first
cargo build --release

# -----------------------------
# Start TO-DO service (8080)
# -----------------------------
cargo run \
  --manifest-path "$ROOT/to-do/networking/Cargo.toml" \
  --features auth-http \
  --release \
  --no-default-features &
TO_DO_PID=$!

echo "Started TO-DO service (PID=$TO_DO_PID)"

# -----------------------------
# Start AUTH service (8081)
# -----------------------------
cargo run \
  --manifest-path "$ROOT/auth/networking/Cargo.toml" \
  --release &
AUTH_PID=$!

echo "Started AUTH service (PID=$AUTH_PID)"

# -----------------------------
# Wait for services to be ready
# -----------------------------
echo "Waiting for services..."

sleep 5

# -----------------------------
# Create user
# -----------------------------
curl -X POST http://127.0.0.1:8081/api/v1/users/create \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test2@gmail.com",
    "password": "password"
}'

echo ""

# -----------------------------
# Login to get token
# -----------------------------
token=$(curl -s \
  -u test@gmail.com:password \
  http://127.0.0.1:8081/api/v1/auth/login)
token=$(echo "$token" | tr -d '\r\n' | sed 's/^"//' | sed 's/"$//')

echo "TOKEN: $token"

# -----------------------------
# Call TO-DO service
# -----------------------------
response=$(curl -s -X POST http://127.0.0.1:8080/api/v1/create \
  -H "Content-Type: application/json" \
  -H "token: $token" \
  -d '{
    "title": "code",
    "status": "PENDING"
}')

echo "Response: $response"

# -----------------------------
# Cleanup
# -----------------------------
echo "Shutting down..."

kill $TO_DO_PID || true
kill $AUTH_PID || true

docker compose down