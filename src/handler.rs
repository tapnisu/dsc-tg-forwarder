use crate::utils::format_message;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use teloxide::prelude::*;
use teloxide::types::ParseMode;

pub struct Handler {
    pub sender_bot: Bot,
    pub output_chat_id: String,

    pub allowed_guilds_ids: Vec<u64>,
    pub muted_guilds_ids: Vec<u64>,

    pub allowed_channels_ids: Vec<u64>,
    pub muted_channels_ids: Vec<u64>,

    pub allowed_users_ids: Vec<u64>,
    pub muted_users_ids: Vec<u64>,

    pub hide_usernames: bool,
}

impl Handler {
    fn check_filters(&self, msg: &Message) -> bool {
        (!self
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
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if self.check_filters(&msg) {
            return;
        }

        if let Err(err) = send_message(self, &ctx, &msg).await {
            println!("{}", err);
        }
    }
}

async fn send_message(
    handler: &Handler,
    ctx: &Context,
    msg: &Message,
) -> anyhow::Result<teloxide::prelude::Message> {
    let message = format_message(ctx, msg, handler.hide_usernames).await?;

    let result = handler
        .sender_bot
        .send_message(handler.output_chat_id.to_owned(), message)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;

    Ok(result)
}
