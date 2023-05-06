use clap::Parser;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::prelude::Guild;
use serenity::prelude::*;
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    /// Discord token (if not used will be taken from DISCORD_TOKEN)
    #[clap(short, long)]
    discord_token: Option<String>,
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!(
            "[{}]: {}",
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
            msg.content_safe(ctx.cache)
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
