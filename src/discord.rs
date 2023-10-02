use std::sync::{Arc, OnceLock};

use serenity::{client::EventHandler, framework::StandardFramework, http::Http, prelude::*};

use crate::config;

static HTTP_CONTEXT: OnceLock<Arc<Http>> = OnceLock::new();

pub async fn create_bot_and_init_http_context(handler: impl EventHandler + 'static) -> Client {
    let config = config::global();

    let framework = StandardFramework::new();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::Client::builder(config.discord_token.clone(), intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    HTTP_CONTEXT
        .set(client.cache_and_http.http.clone())
        .expect("Failed to set HTTP context");

    client
}

pub fn get_http_context() -> &'static Arc<Http> {
    HTTP_CONTEXT
        .get()
        .expect("Tried to get HTTP context before it was set")
}
