mod cli;
mod config;
mod utils;

use crate::config::parse_config;
use clap::Parser;
use cli::Cli;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use utils::format_message;

struct Handler {
    bot: Bot,
    output_chat_id: String,

    allowed_guilds_ids: Vec<u64>,
    muted_guilds_ids: Vec<u64>,

    allowed_channels_ids: Vec<u64>,
    muted_channels_ids: Vec<u64>,

    allowed_users_ids: Vec<u64>,
    muted_users_ids: Vec<u64>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if (!self
            .allowed_guilds_ids
            .contains(&msg.guild_id.unwrap_or_default().0)
            && !self.allowed_guilds_ids.is_empty())
            || (self
                .muted_guilds_ids
                .contains(&msg.guild_id.unwrap_or_default().0)
                && !self.muted_guilds_ids.is_empty())
            || (!self.allowed_channels_ids.contains(&msg.channel_id.0)
                && !self.allowed_channels_ids.is_empty())
            || (self.muted_channels_ids.contains(&msg.channel_id.0)
                && !self.muted_channels_ids.is_empty())
            || (!self.allowed_users_ids.contains(&msg.author.id.0)
                && !self.allowed_users_ids.is_empty())
            || (self.muted_users_ids.contains(&msg.author.id.0) && !self.muted_users_ids.is_empty())
        {
            return;
        }

        self.bot
            .send_message(self.output_chat_id.clone(), format_message(ctx, msg).await)
            .parse_mode(ParseMode::MarkdownV2)
            .await
            .unwrap();
    }
}

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
    let mut client = Client::builder(discord_token)
        .event_handler(Handler {
            bot: Bot::new(telegram_token),
            output_chat_id,
            allowed_guilds_ids: config.allowed_guilds_ids,
            muted_guilds_ids: config.muted_guilds_ids,
            allowed_channels_ids: config.allowed_channels_ids,
            muted_channels_ids: config.muted_channels_ids,
            allowed_users_ids: config.allowed_users_ids,
            muted_users_ids: config.muted_users_ids,
        })
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
