# Overherd P2P Forum

<img src="https://avatars.githubusercontent.com/u/224887258?s=300&u=8804756b9a8dc8e9eeb15c614b5f89d5a64dc561&v=4" alt="Overherd" width=300>

## Development

### Local

```bash
export OVERHEARD_CONFIG_PATH="/path/to/config.toml" # simply config.toml for local testing
```

### Docker

Startup

```bash
# Starts multiple instances
docker compose up
```

Connect to instances

```bash
docker compose exec node sh # connect to first node
docker exec -it overherd-node-2 sh # connect to other nodes

# To send a command right away you can use:
docker compose exec -T node nc 127.0.0.1 9999 <<< "PUBL 00000006 hello"
docker exec -i overherd-node-2 nc 127.0.0.1 9999 <<< "PUBL 00000006 hello"
```

