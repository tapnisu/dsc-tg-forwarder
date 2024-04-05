use crate::Cli;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub discord_token: Option<String>,
    pub telegram_token: Option<String>,
    pub output_chat_id: Option<String>,

    #[serde(default)]
    pub allowed_guilds_ids: Vec<u64>,
    #[serde(default)]
    pub muted_guilds_ids: Vec<u64>,

    #[serde(default)]
    pub allowed_channels_ids: Vec<u64>,
    #[serde(default)]
    pub muted_channels_ids: Vec<u64>,

    #[serde(default)]
    pub allowed_users_ids: Vec<u64>,
    #[serde(default)]
    pub muted_users_ids: Vec<u64>,

    #[serde(default)]
    pub hide_usernames: bool,
}

pub fn parse_config(path: &PathBuf) -> anyhow::Result<Config> {
    if !path.exists() {
        let js = Config {
            discord_token: Some("discord-token".to_owned()),
            telegram_token: Some("telegram-token".to_owned()),
            output_chat_id: Some("telegram-chat-id-to-output-messages".to_owned()),

            allowed_guilds_ids: vec![],
            muted_guilds_ids: vec![],

            allowed_channels_ids: vec![],
            muted_channels_ids: vec![],

            allowed_users_ids: vec![],
            muted_users_ids: vec![],

            hide_usernames: true,
        };
        let yaml = serde_yaml::to_string(&js)?;

        let parent = path.parent().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Parent directory not found",
        ))?;
        fs::create_dir_all(parent)?;
        File::create(path)?.write_all(yaml.as_bytes())?;
    }

    let yaml = fs::read_to_string(path)?;

    let config = serde_yaml::from_str(&yaml)?;

    Ok(config)
}

// Todo: Return struct instead of tuple
pub fn get_tokens(args: &Cli, config: &Config) -> anyhow::Result<(String, String, String)> {
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
