# Solana TPS BOT

[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.svg?v=103)](https://github.com/ellerbrock/open-source-badges/)
[![deploy](https://github.com/DerZwergGimli/solana_tps_bot/actions/workflows/docker-publish.yml/badge.svg)](https://github.com/DerZwergGimli/SolanaStatus_BOT/actions/workflows/docker-publish.yml)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![badge_discord](https://badgen.net/badge/icon/discord?icon=discord&label)
![badge_docker_size](https://badgen.net/docker/pulls/derzwerggimli/solana_tps_bot)
![badge_docker_size](https://badgen.net/docker/size/derzwerggimli/solana_tps_bot)

This repo contains a **Discord** bot that is **monitoring** the **SolanaBlockchain**.

!["bot icon""](icon.drawio.png)

## Features:

- Display current TPS (transactions per second)
- Command `!tps` will print out the TPS
- Connection to RPC

## View

!["bot View1""](bot_view.png)

!["bot view2""](dc_view1.png)

## Discord-Settings:

- ### BOT Permissions
    - Manage Role
    - Change Nickname
- ### BOT Roles
    - tickers-red
    - tickers-green

Create 2 roles in you discord server so the bot will change its nickname color based on that.

!["bot roles""](dc_roles.png)

## Deploy via Docker

- Copy `.env.sample` to `.env`
- Add your access tokens and config to `.env`
- Start you BOT with `docker-compose up -d`

### ENV

- Required:

```gitignore
RUST_LOG=error
DISCORD_TOKEN=<DISCORD_TOKEN>
RPC_ENDPOINT=<RPC_ENDPOINT>
LOOP_UPDATE_SLEEP=60
TPS_THRESHOLD=2000
```

## Donations:

- Solana-Wallet: `coffeeplease.sol`
- ETH-Address: `0xB0Be2420cA00C86aD983F246DEF49EA0F9779DCd`
