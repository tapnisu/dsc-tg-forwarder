use clap::{error::ErrorKind, CommandFactory, Parser};
use dsc_tg_forwarder::{cli::Cli, config::parse_config, handler::Handler, Config};
use serenity::prelude::*;
use std::{env, error::Error, fmt::Display};
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
    let (discord_token, telegram_token, output_chat_id) = get_tokens(&args.clone(), &config)
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

// Todo: Return struct instead of tuple
fn get_tokens(args: &Cli, config: &Config) -> anyhow::Result<(String, String, String)> {
    let discord_token = get_discord_token(args, config)?;
    let telegram_token = get_telegram_token(args, config)?;
    let output_channel = get_output_channel(args, config)?;

    Ok((discord_token, telegram_token, output_channel))
}

fn get_telegram_token(args: &Cli, config: &Config) -> anyhow::Result<String> {
    let telegram_token = match (
        args.telegram_token.clone(),
        config.telegram_token.clone(),
        env::var("TELEGRAM_TOKEN"),
    ) {
        (Some(telegram_token), _, _) => telegram_token,
        (_, Some(telegram_token), _) => telegram_token,
        (_, _, Ok(telegram_token)) => telegram_token,
        (None, None, Err(_)) => return Err(ConfigTokenError("Telegram token".to_owned()).into()),
    };

    Ok(telegram_token)
}

fn get_discord_token(args: &Cli, config: &Config) -> anyhow::Result<String> {
    let discord_token = match (
        args.discord_token.clone(),
        config.discord_token.clone(),
        env::var("DISCORD_TOKEN"),
    ) {
        (Some(discord_token), _, _) => discord_token,
        (_, Some(discord_token), _) => discord_token,
        (_, _, Ok(discord_token)) => discord_token,
        (None, None, Err(_)) => return Err(ConfigTokenError("Discord token".to_owned()).into()),
    };

    Ok(discord_token)
}

fn get_output_channel(args: &Cli, config: &Config) -> anyhow::Result<String> {
    let output_chat_id = match (
        args.output_chat_id.clone(),
        config.output_chat_id.clone(),
        env::var("OUTPUT_CHAT_ID"),
    ) {
        (Some(output_chat_id), _, _) => output_chat_id,
        (_, Some(output_chat_id), _) => output_chat_id,
        (_, _, Ok(output_chat_id)) => output_chat_id,
        (None, None, Err(_)) => return Err(ConfigTokenError("Output channel".to_owned()).into()),
    };

    Ok(output_chat_id)
}

#[derive(Clone, Debug)]
struct ConfigTokenError(String);

impl Display for ConfigTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} not found!", self.0)
    }
}

impl Error for ConfigTokenError {}
