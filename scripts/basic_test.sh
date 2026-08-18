#!/usr/bin/env bash

SCRIPTPATH="$(
	cd "$(dirname "$0")"
	pwd -P
)"
cd $SCRIPTPATH
cd ..

echo "Building and running Actix server..."
cargo build -p to-do-actix-server
cargo run -p to-do-actix-server &
PID=$!

sleep 1

rm tasks.json
rm output.txt
cat <<EOF >tasks.json
{}
EOF

curl -X POST http://127.0.0.1:8080/api/v1/create \
	-H "Content-Type: application/json" \
	-d '{"title": "writing", "status": "PENDING"}' >>output.txt
echo "" >>output.txt
curl -X POST http://127.0.0.1:8080/api/v1/create \
	-H "Content-Type: application/json" \
	-d '{"title": "coding", "status": "PENDING"}' >>output.txt
echo "" >>output.txt
curl -X DELETE http://127.0.0.1:8080/api/v1/delete/coding >>output.txt
echo "" >>output.txt
curl -X PATCH http://127.0.0.1:8080/api/v1/update \
	-H "Content-Type: application/json" \
	-d '{"title": "writing", "status": "DONE"}' \
	>>output.txt
kill $PID
