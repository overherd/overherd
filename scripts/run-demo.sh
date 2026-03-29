#!/bin/bash

docker exec overherd-node-1 curl -s localhost:9999/join?peer=overherd-node-2
docker exec overherd-node-2 curl -s localhost:9999/join?peer=overherd-node-3
docker exec overherd-node-3 curl -s localhost:9999/join?peer=overherd-node-4
docker exec overherd-node-4 curl -s localhost:9999/join?peer=overherd-node-5
docker exec overherd-node-5 curl -s localhost:9999/join?peer=overherd-node-6
docker exec overherd-node-6 curl -s localhost:9999/join?peer=overherd-node-7
docker exec overherd-node-7 curl -s localhost:9999/join?peer=overherd-node-8
docker exec overherd-node-8 curl -s localhost:9999/join?peer=overherd-node-9
docker exec overherd-node-9 curl -s localhost:9999/join?peer=overherd-node-10
docker exec overherd-node-10 curl -s localhost:9999/join?peer=overherd-node-11
docker exec overherd-node-11 curl -s localhost:9999/join?peer=overherd-node-12
docker exec overherd-node-12 curl -s localhost:9999/join?peer=overherd-node-13
docker exec overherd-node-13 curl -s localhost:9999/join?peer=overherd-node-14
docker exec overherd-node-14 curl -s localhost:9999/join?peer=overherd-node-15
docker exec overherd-node-15 curl -s localhost:9999/join?peer=overherd-node-16
docker exec overherd-node-16 curl -s localhost:9999/join?peer=overherd-node-17
docker exec overherd-node-17 curl -s localhost:9999/join?peer=overherd-node-18
docker exec overherd-node-18 curl -s localhost:9999/join?peer=overherd-node-19
docker exec overherd-node-19 curl -s localhost:9999/join?peer=overherd-node-20
docker exec overherd-node-20 curl -s localhost:9999/join?peer=overherd-node-1
# docker exec overherd-node-2 curl -s localhost:9999/broadcast?data=hello
