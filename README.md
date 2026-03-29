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
# Starts multiple instances (build is required to rebuild from source changes)
docker compose up --build
```

Connect to instances

```bash
docker exec -it overherd-node-2 sh # connect to nodes

# To send a command right away you can use:
docker exec overherd-node-1 curl -s localhost:9999/join?peer=overherd-node-2
```
