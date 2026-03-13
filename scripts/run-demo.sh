#!/bin/bash

docker exec overherd-node-1 curl -s localhost:9999/join?peer=overherd-node-2
docker exec overherd-node-2 curl -s localhost:9999/join?peer=overherd-node-3
docker exec overherd-node-4 curl -s localhost:9999/join?peer=overherd-node-2
docker exec overherd-node-5 curl -s localhost:9999/join?peer=overherd-node-3
docker exec overherd-node-2 curl -s localhost:9999/broadcast?data=hello
