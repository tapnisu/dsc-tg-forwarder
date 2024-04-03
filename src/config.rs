use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

fn empty_ids_vec() -> Vec<u64> {
    vec![]
}

fn default_false() -> bool {
    false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub discord_token: Option<String>,
    pub telegram_token: Option<String>,
    pub output_chat_id: Option<String>,

    #[serde(default = "empty_ids_vec")]
    pub allowed_guilds_ids: Vec<u64>,
    #[serde(default = "empty_ids_vec")]
    pub muted_guilds_ids: Vec<u64>,

    #[serde(default = "empty_ids_vec")]
    pub allowed_channels_ids: Vec<u64>,
    #[serde(default = "empty_ids_vec")]
    pub muted_channels_ids: Vec<u64>,

    #[serde(default = "empty_ids_vec")]
    pub allowed_users_ids: Vec<u64>,
    #[serde(default = "empty_ids_vec")]
    pub muted_users_ids: Vec<u64>,

    #[serde(default = "default_false")]
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
        let yaml = serde_yaml::to_string(&js).unwrap();

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
