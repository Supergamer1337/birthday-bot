mod commands;
mod config;
mod discord;
mod event_handler;
mod storage;

#[tokio::main]
async fn main() {
    let handler = event_handler::new().await;
    let mut bot = discord::create_bot(handler).await;

    if let Err(why) = bot.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
