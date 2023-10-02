mod commands;
mod config;
mod discord;
mod event_handler;
mod reminders;
mod storage;

#[tokio::main]
async fn main() {
    let storage = storage::SqliteStorage::arc()
        .await
        .expect("Failed to create storage");

    let handler = event_handler::new(storage.clone()).await;
    let mut bot = discord::create_bot_and_init_http_context(handler).await;

    reminders::schedule_tasks(storage.clone()).await;

    if let Err(why) = bot.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
