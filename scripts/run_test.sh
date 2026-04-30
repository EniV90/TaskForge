#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P)"
cd $SCRIPTPATH

cd ..

if [ -d "./logs" ]; then
  echo "Removing existing log directory : ./logs"
  rm -rf "./logs"
fi
mkdir logs

function check_postgres_ready() {
  docker compose exec -T postgres pg_isready \
  -U username -d to_do
}

export $(cat .env | xargs)

cargo test -p to-do-core > ./logs/to-do-core.log
cargo test -p to-do-networking-server > ./logs/to-do-networking-server.log

Docker compose up -d

# Wait for postgres to be ready 
echo "Waiting for postgres to be ready..."
until check_postgres_ready; do 
  echo -n "."
  sleep 1
done

echo "Postgres is ready."

export TO_DO_MAX_CONNECTIONS=1
cargo test -p to-do-dal --features sqlx-postgres \
-- --test-threads=1 > ./logs/to-do-dal.log

Docker compose down