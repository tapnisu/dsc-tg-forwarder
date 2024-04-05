use clap::{error::ErrorKind, CommandFactory, Parser};
use dsc_tg_forwarder::{cli::Cli, config, config::parse_config, handler::Handler};
use serenity::prelude::*;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let mut cmd = Cli::command();

    let config_path = args.config_path.clone().unwrap_or_else(|| {
        format!(
            "{}/.config/dsc-tg-forwarder/config.yml",
            home::home_dir().unwrap().display()
        )
    });

    let config = parse_config(&config_path.into())
        .unwrap_or_else(|err| cmd.error(ErrorKind::InvalidValue, err).exit());
    let (discord_token, telegram_token, output_chat_id) =
        config::get_tokens(&args.clone(), &config)
            .unwrap_or_else(|err| cmd.error(ErrorKind::InvalidValue, err).exit());

    // Login with a bot token from the environment
    let mut client = Client::builder(discord_token)
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
        .await
        .unwrap_or_else(|err| cmd.error(ErrorKind::InvalidValue, err).exit());

    if let Err(err) = client.start().await {
        cmd.error(ErrorKind::InvalidValue, err).exit()
    }
}
