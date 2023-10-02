use std::sync::Arc;

use chrono::NaiveDate;
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

    let date = match options[1].value {
        Some(ref date) => date.to_string().replace("\"", ""),
        None => return "No date provided".to_string(),
    };

    let date = match NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d") {
        Ok(date) => date,
        Err(why) => {
            println!("Failed to parse date: {}", why);
            return "Date format invalid. Should be YYYY-MM-DD".to_string();
        }
    };

    match storage.add_birthday(&name, date).await {
        Ok(_) => format!("Added birthday for {}", name),
        Err(_) => "Failed to add birthday".to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("add_birthday")
        .description("Add a birthday")
        .create_option(|option| {
            option
                .name("name")
                .description("Name of the person")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("date")
                .description("Date of the birthday")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
