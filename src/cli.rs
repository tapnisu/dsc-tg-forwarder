use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Discord token (if not used will be taken from DISCORD_TOKEN)
    #[clap(short, long)]
    pub discord_token: Option<String>,
    /// Telegram token (if not used will be taken from TELEGRAM_TOKEN)
    #[clap(short, long)]
    pub telegram_token: Option<String>,
    /// ID of telegram user/group to send output to
    #[clap(short, long)]
    pub output_chat_id: Option<String>,
    /// Path to configuration file (default is ~/.config/dsc-tg-forwarder/config.yml)
    #[clap(short, long)]
    pub config_path: Option<String>,
}
