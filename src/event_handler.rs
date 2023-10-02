use crate::{
    commands,
    storage::{self, Storage},
};
use serenity::model::application::command::Command;
use serenity::{async_trait, model::prelude::*, prelude::*};
use std::sync::Arc;

pub struct Handler {
    storage: Arc<dyn Storage>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "add_birthday" => {
                    commands::add_birthday::run(&command.data.options, self.storage.clone()).await
                }
                _ => "Not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why)
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let registered_commands =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::add_birthday::register(command)
            })
            .await;

        if let Err(why) = registered_commands {
            println!("Failed to register slash commands: {}", why);
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
