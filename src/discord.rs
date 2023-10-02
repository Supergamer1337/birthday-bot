use serenity::{client::EventHandler, framework::StandardFramework, prelude::*};

use crate::config;

pub async fn create_bot(handler: impl EventHandler + 'static) -> Client {
    let config = config::global();

    let framework = StandardFramework::new();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    serenity::Client::builder(config.discord_token.clone(), intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client")
}
