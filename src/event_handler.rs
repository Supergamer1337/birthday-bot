use crate::storage::{self, Storage};
use chrono::NaiveDate;
use serenity::{async_trait, model::prelude::*, prelude::*};
use std::sync::Arc;

pub struct Handler {
    storage: Arc<dyn Storage>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("birthday add") {
            let content = msg.content.replace("birthday add ", "");
            let mut split = content.split(" ");
            let name = split.next().unwrap();
            let date = split.next().unwrap();

            let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();

            if let Err(why) = self.storage.add_birthday(name, date).await {
                println!("Error adding birthday: {:?}", why);
            }

            if let Err(why) = msg.channel_id.say(&ctx.http, "Birthday added").await {
                println!("Error sending message: {:?}", why);
                return;
            }
        }

        if msg.content == "birthdays" {
            let birthdays = self.storage.get_birthdays().await.unwrap();

            let mut response = String::from("Birthdays:\n");

            for (name, date) in birthdays {
                response.push_str(&format!("{}: {}\n", name, date));
            }

            if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

pub async fn new() -> Handler {
    let storage = storage::SqliteStorage::new()
        .await
        .expect("Failed to create storage");

    Handler {
        storage: Arc::new(storage),
    }
}
