use clap::{error::ErrorKind, CommandFactory, Parser};
use dsc_tg_forwarder::{cli::Cli, config::parse_config, handler::Handler};
use serenity::prelude::*;
use std::env;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let config_path = args.config_path.unwrap_or_else(|| {
        format!(
            "{}/.config/dsc-tg-forwarder/config.yml",
            home::home_dir().unwrap().display()
        )
    });
    let config = parse_config(&config_path.into())
        .unwrap_or_else(|err| Cli::command().error(ErrorKind::InvalidValue, err).exit());

    let discord_token = match (
        args.discord_token,
        config.discord_token,
        env::var("DISCORD_TOKEN"),
    ) {
        (Some(discord_token), _, _) => discord_token,
        (_, Some(discord_token), _) => discord_token,
        (_, _, Ok(discord_token)) => discord_token,
        (None, None, Err(_)) => Cli::command()
            .error(ErrorKind::InvalidValue, "Discord token wasn't supplied")
            .exit(),
    };

    let telegram_token = match (
        args.telegram_token,
        config.telegram_token,
        env::var("TELEGRAM_TOKEN"),
    ) {
        (Some(telegram_token), _, _) => telegram_token,
        (_, Some(telegram_token), _) => telegram_token,
        (_, _, Ok(telegram_token)) => telegram_token,
        (None, None, Err(_)) => Cli::command()
            .error(ErrorKind::InvalidValue, "Telegram token wasn't supplied")
            .exit(),
    };

    let output_chat_id = match (
        args.output_chat_id,
        config.output_chat_id,
        env::var("OUTPUT_CHAT_ID"),
    ) {
        (Some(output_chat_id), _, _) => output_chat_id,
        (_, Some(output_chat_id), _) => output_chat_id,
        (_, _, Ok(output_chat_id)) => output_chat_id,
        (None, None, Err(_)) => Cli::command()
            .error(ErrorKind::InvalidValue, "Output chat id wasn't supplied")
            .exit(),
    };

    // Login with a bot token from the environment
    let client = Client::builder(discord_token)
        .event_handler(Handler {
            sender_bot: Bot::new(telegram_token),
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
