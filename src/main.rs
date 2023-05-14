mod config;
mod utils;

use crate::config::parse_config;
use clap::Parser;
use home;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use utils::format_message;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Discord token (if not used will be taken from DISCORD_TOKEN)
    #[clap(short, long)]
    discord_token: Option<String>,
    /// Telegram token (if not used will be taken from TELEGRAM_TOKEN)
    #[clap(short, long)]
    telegram_token: Option<String>,
    /// Id of telegram user/channel to send output to
    #[clap(short, long)]
    output_channel_id: Option<String>,
    /// Path to configuration file (default is ~/.config/dsc-tg-forwarder/config.yml)
    #[clap(short, long)]
    config_path: Option<String>,
}

struct Handler {
    bot: Bot,
    output_channel_id: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        self.bot
            .send_message(
                self.output_channel_id.clone(),
                format_message(ctx, msg).await,
            )
            .parse_mode(ParseMode::MarkdownV2)
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let config = parse_config(args.config_path.unwrap_or(format!(
        "{}/.config/dsc-tg-forwarder/config.yml",
        home::home_dir().unwrap().display()
    )));

    // Login with a bot token from the environment
    let mut client =
        Client::builder(args.discord_token.unwrap_or(
            config.discord_token.unwrap_or_else(|| {
                env::var("DISCORD_TOKEN").expect("Discord token wasn't supplied")
            }),
        ))
        .event_handler(Handler {
            bot: Bot::new(
                args.telegram_token
                    .unwrap_or(config.telegram_token.unwrap_or_else(|| {
                        env::var("TELEGRAM_TOKEN").expect("Telegram token wasn't supplied")
                    })),
            ),
            output_channel_id: args.output_channel_id.unwrap_or(
                config.output_channel_id.unwrap_or_else(|| {
                    env::var("OUTPUT_CHANNEL_ID").expect("Output channel wasn't supplied")
                }),
            ),
        })
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
