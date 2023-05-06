use clap::Parser;
use serenity::async_trait;
use serenity::model::channel::Message;
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
    async fn message(&self, ctx: Context, message: Message) {
        println!(
            "[{}]: {}",
            message.author.tag(),
            message.content_safe(ctx.cache)
        )
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
