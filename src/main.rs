mod config;

use serenity::async_trait;
use serenity::client::EventHandler;
use serenity::framework::StandardFramework;
use serenity::model::prelude::Message;
use serenity::prelude::{Context, GatewayIntents};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = config::global();

    let framework = StandardFramework::new();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::Client::builder(config.discord_token.clone(), intents)
        .event_handler(Handler {})
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
