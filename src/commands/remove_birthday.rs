use std::sync::Arc;

use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{application_command::CommandDataOption, command::CommandOptionType},
};

use crate::storage::Storage;

pub async fn run(options: &[CommandDataOption], storage: Arc<dyn Storage>) -> String {
    let name = match options[0].value {
        Some(ref name) => name.to_string().replace("\"", ""),
        None => return "No name provided".to_string(),
    };

    match storage.remove_birthday(&name).await {
        Ok(_) => format!("Removed birthday for {}", name),
        Err(_) => "Failed to remove birthday".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove_birthday")
        .description("Remove a birthday")
        .create_option(|option| {
            option
                .name("name")
                .description("Name of the person whose birthday you want to remove")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
