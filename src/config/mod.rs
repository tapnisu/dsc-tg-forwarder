use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub discord_token: Option<String>,
    pub telegram_token: Option<String>,
    pub output_channel_id: Option<String>,
}

pub fn parse_config(file_path: impl AsRef<Path>) -> Config {
    let yaml = fs::read_to_string(file_path).unwrap();

    serde_yaml::from_str(&yaml).unwrap()
}
