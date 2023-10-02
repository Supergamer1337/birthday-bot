use std::sync::Arc;

use serenity::{
    builder::CreateApplicationCommand, model::prelude::application_command::CommandDataOption,
};

use crate::storage::{Birthday, Storage};

pub async fn run(_options: &[CommandDataOption], storage: Arc<dyn Storage>) -> String {
    match storage.get_birthdays().await {
        Ok(list) => {
            if list.is_empty() {
                return "No birthdays stored".to_string();
            }

            let mut result = "Birthdays:\n".to_string();

            for Birthday(name, date) in list {
                result.push_str(format!("{}: {}\n", name, date).as_str());
            }

            result
        }
        Err(_) => "Failed to remove birthday".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list_birthdays")
        .description("List all currently stored birthdays")
}
