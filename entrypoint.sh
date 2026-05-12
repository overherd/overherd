#!/bin/sh
# Docker entrypoint to be able to store data for each replica
export OVERHERD__DATA__LIST_NAME="${HOSTNAME##*-}.dat"
export OVERHERD__DATA__LIST_PATH=/overherd/output
exec "$@"
