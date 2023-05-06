use clap::Parser;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::prelude::{Embed, Guild};
use serenity::prelude::*;
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Discord token (if not used will be taken from DISCORD_TOKEN)
    #[clap(short, long)]
    discord_token: Option<String>,
}

fn format_embed(embed: Embed) -> String {
    let mut res = "".to_owned();

    if let Some(title) = embed.title {
        res += &format!("Title: {}\n", title);
    }
    if let Some(description) = embed.description {
        res += &format!("Description: {}\n", description);
    }
    if let Some(url) = embed.url {
        res += &format!("Url: {}\n", url);
    }

    res += &embed
        .fields
        .iter()
        .map(|f| format!("\n  {}\n  {}\n", f.name, f.value))
        .collect::<String>();

    if let Some(thumbnail) = embed.thumbnail {
        res += &format!("Thumbnail: {}\n", thumbnail.url);
    }
    if let Some(image) = embed.image {
        res += &format!("Image: {}\n", image.url);
    }
    if let Some(video) = embed.video {
        res += &format!("Video: {}\n", video.url);
    }
    if let Some(author) = embed.author {
        res += &format!("Author: {}\n", author.name);
    }
    if let Some(footer) = embed.footer {
        res += &format!("Footer: {}\n", footer.text);
    }
    if let Some(timestamp) = embed.timestamp {
        res += &format!("Timestamp: {}\n", timestamp);
    }

    res
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        print!(
            "[{}]: {}\n{}",
            if msg.is_private() {
                msg.author.tag()
            } else {
                format!(
                    "{} / #{} / {}",
                    Guild::get(&ctx, msg.guild_id.unwrap()).await.unwrap().name,
                    msg.channel_id
                        .to_channel(&ctx.http)
                        .await
                        .unwrap()
                        .guild()
                        .unwrap()
                        .name,
                    msg.author.tag(),
                )
            },
            msg.content_safe(ctx.cache),
            msg.embeds
                .into_iter()
                .map(|e| format_embed(e))
                .collect::<String>()
        );
    }
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    // Login with a bot token from the environment
    let mut client = Client::builder(
        args.discord_token
            .unwrap_or_else(|| env::var("DISCORD_TOKEN").expect("Discord token wasn't supplied")),
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
