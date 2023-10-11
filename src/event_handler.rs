use crate::{commands, storage::BirthdayStorage};
use serenity::{
    async_trait,
    model::prelude::{application_command::ApplicationCommandInteraction, command::Command, *},
    prelude::*,
};
use std::sync::Arc;

pub struct Handler {
    storage: Arc<dyn BirthdayStorage>,
}

async fn handle_command(command: &ApplicationCommandInteraction, handler: &Handler) -> String {
    match command.data.name.as_str() {
        "add_birthday" => {
            commands::add_birthday::run(&command.data.options, handler.storage.clone()).await
        }
        "remove_birthday" => {
            commands::remove_birthday::run(&command.data.options, handler.storage.clone()).await
        }
        "list_birthdays" => {
            commands::list_birthdays::run(&command.data.options, handler.storage.clone()).await
        }
        _ => "Not implemented :(".to_string(),
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = handle_command(&command, &self).await;

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

        let registered_commands = Command::set_global_application_commands(&ctx.http, |command| {
            command
                .create_application_command(|command| commands::add_birthday::register(command))
                .create_application_command(|command| commands::remove_birthday::register(command))
                .create_application_command(|command| commands::list_birthdays::register(command))
        })
        .await;

        if let Err(why) = registered_commands {
            println!("Failed to register slash commands: {}", why);
        }
    }
}

pub async fn new(storage: Arc<dyn BirthdayStorage>) -> Handler {
    Handler { storage }
}
