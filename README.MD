# Solana TPS BOT

[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![buid-test](https://github.com/DerZwergGimli/solana_tps_bot/actions/workflows/rust.yml/badge.svg)](https://github.com/DerZwergGimli/SolanaStatus_BOT/actions/workflows/rust.yml)
[![deploy-to-dockerhub](https://github.com/DerZwergGimli/solana_tps_bot/actions/workflows/docker.yml/badge.svg)](https://github.com/DerZwergGimli/SolanaStatus_BOT/actions/workflows/docker.yml)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![badge_discord](https://badgen.net/badge/icon/discord?icon=discord&label)
![badge_docker_size](https://badgen.net/docker/pulls/derzwerggimli/solana_tps_bot)
![badge_docker_size](https://badgen.net/docker/size/derzwerggimli/solana_tps_bot)

This repo contains a Discord bot that is monitoring the SolanaBlockchain.

!["bot icon""](icon.drawio.png)

## Current Feature:

- Display current tps (Transactions per Second)

## View

!["bot View1""](dc_view2.png)

!["bot view2""](dc_view1.png)

## Discord-Settings:

- ### BOT Permissions
    - Manage Role
    - Change Nickname
- ### BOT Roles
    - tickers-red
    - tickers-green

Create 2 Roles in you Discord Server so the bot will change its Nickname color based on that.

!["bot roles""](dc_roles.png)

### ENV

- Required:

```gitignore
RUST_LOG=error
DISCORD_TOKEN=<DISCORD_TOKEN>
RPC_ENDPOINT=<RPC_ENDPOINT>
LOOP_UPDATE_SLEEP=5
TPS_THRESHOLD=2000
```

## Deploy via Docker

- Copy the `docker-compose.yaml.sample` to `docker-compose.yaml`
- Add you <DISCORD_TOKEN>
- Start you BOT with `docker-compose up -d`

### Donations:

- Solana-Wallet: `coffeeplease.sol`
- ETH-Address: `0xB0Be2420cA00C86aD983F246DEF49EA0F9779DCd`
