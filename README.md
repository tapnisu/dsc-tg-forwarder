# dsc-tg-forwarder

Selfbot to send your incoming Discord messages to Telegram

> **Warning**:
> Selfbots are against Discord's Terms of Service, use at your own risk!

## Requirements

1. [Rust](https://www.rust-lang.org/tools/install)

## Installation

1. Run in your shell:

```sh
cargo install --git https://github.com/tapnisu/dsc-tg-forwarder
```

## Usage

### Using config

1. Run `dsc-tg-forwarder`, which will generate default config at [~/.config/dsc-tg-forwarder/config.yml](assets/config.yml)

2. Config it

3. Run `dsc-tg-forwarder`

### Using environment variables

1. Input your Discord token into DISCORD_TOKEN environment variable

2. Input your Telegram bot token into TELEGRAM_TOKEN environment variable

3. Input your Output channel id into OUTPUT_CHAT_ID environment variable

4. Run `dsc-tg-forwarder`

### Using cli

Just run this in your shell:

```sh
dsc-tg-forwarder -d YOUR_DISCORD_TOKEN -t YOUR_TELEGRAM_TOKEN -o OUTPUT_CHAT_ID
```
