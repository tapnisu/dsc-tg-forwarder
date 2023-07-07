use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn empty_ids_vec() -> Vec<u64> {
    vec![]
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
}

pub fn parse_config(path: String) -> Config {
    let path = Path::new(&path);

    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        File::create(path)
            .expect("Failed to create config file")
            .write_all(include_str!("../assets/config.yml").as_bytes())
            .expect("Failed to write default config file");
    }

    let yaml = fs::read_to_string(path).expect("Failed to read config");

    serde_yaml::from_str(&yaml).expect("Failed to read config")
}
