mod cli;
mod config;
mod handler;
mod utils;

use clap::{error::ErrorKind, CommandFactory, Parser};
use cli::Cli;
use config::parse_config;
use handler::Handler;
use serenity::prelude::*;
use std::env;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let config_path = args.config_path.unwrap_or(format!(
        "{}/.config/dsc-tg-forwarder/config.yml",
        home::home_dir().unwrap().display()
    ));
    let config = parse_config(config_path);

    let discord_token = args.discord_token.unwrap_or(
        config
            .discord_token
            .unwrap_or_else(|| env::var("DISCORD_TOKEN").expect("Discord token wasn't supplied")),
    );

    let telegram_token =
        args.telegram_token
            .unwrap_or(config.telegram_token.unwrap_or_else(|| {
                env::var("TELEGRAM_TOKEN").expect("Telegram token wasn't supplied")
            }));

    let output_chat_id =
        args.output_chat_id
            .unwrap_or(config.output_chat_id.unwrap_or_else(|| {
                env::var("OUTPUT_CHAT_ID").expect("Output chat id wasn't supplied")
            }));

    // Login with a bot token from the environment
    let client = Client::builder(discord_token)
        .event_handler(Handler {
            bot: Bot::new(telegram_token),
            output_chat_id,
            allowed_guilds_ids: config.allowed_guilds_ids,
            muted_guilds_ids: config.muted_guilds_ids,
            allowed_channels_ids: config.allowed_channels_ids,
            muted_channels_ids: config.muted_channels_ids,
            allowed_users_ids: config.allowed_users_ids,
            muted_users_ids: config.muted_users_ids,
            hide_usernames: config.hide_usernames,
        })
        .await;

    match client {
        Err(err) => Cli::command().error(ErrorKind::InvalidValue, err).exit(),
        Ok(mut client) => {
            // start listening for events by starting a single shard
            if let Err(err) = client.start().await {
                let _ = Cli::command().error(ErrorKind::InvalidValue, err).print();
            }
        }
    }
}
