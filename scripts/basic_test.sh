# #!/usr/bin/env bash

# # navigate to directory
# SCRIPTPATH="$(cd "$(dirname "$0")"; pwd -p)"
# cd $SCRIPTPATH

# cd ..

# framework=${1:-actix}

# # Build and run
# case $framework in actix)
# echo "Building and Running server..."
# cargo build -p to-do-networking-server
# cargo run -p to-do-networking-server &
# PID =$!
# ;;

# esac
# sleep 1


# rm tasks.json
# rm output.txt
# cat<<EOF > tasks.json
# {}
# EOF


# curl -X POST http://127.0.0.1:8080/api/v1/create \
# -H "Content-Type: application/json" \
# -d '{"title": "gaming", "status": "PENDING"}' >> output.txt
# echo "" >> output.txt

# curl -X POST http://127.0.0.1:8080/api/v1/create \
# -H "Content-Type: application/json" \
# -d '{"title": "driving", "status": "PENDING"}' >> output.txt
# echo "" >> output.txt

# curl -X DELETE http://127.0.0.1:8080/api/v1/delete/coding >> output.txt
# kill $PID


#!/usr/bin/env bash

set -e  # stop on error

# Go to project root (VERY IMPORTANT)
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR/.."

echo "Current dir: $(pwd)"

# Build and run server
echo "Starting server..."
cargo run -p to-do-networking-server &
PID=$!

# Wait for server to start
sleep 3

# Reset files
rm -f tasks.json output.txt
echo "{}" > tasks.json

# Create tasks
curl -X POST http://127.0.0.1:8080/api/v1/create \
  -H "Content-Type: application/json" \
  -d '{"title": "gaming", "status": "PENDING"}' >> output.txt

echo "" >> output.txt

curl -X POST http://127.0.0.1:8080/api/v1/create \
  -H "Content-Type: application/json" \
  -d '{"title": "driving", "status": "PENDING"}' >> output.txt

echo "" >> output.txt

curl -X POST http://127.0.0.1:8080/api/v1/create \
  -H "Content-Type: application/json" \
  -d '{"title": "coding", "status": "DONE"}' >> output.txt

echo "" >> output.txt

# Delete task
curl -X DELETE http://127.0.0.1:8080/api/v1/delete/coding >> output.txt

# Kill server
kill $PID