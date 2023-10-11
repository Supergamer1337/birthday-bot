use crate::{
    config, discord,
    storage::{Birthday, BirthdayStorage},
};
use chrono::{Datelike, NaiveDate};
use clokwerk::{AsyncScheduler, TimeUnits};
use moka::sync::Cache;
use serenity::model::prelude::ChannelId;
use std::{
    ops::Add,
    sync::{Arc, OnceLock},
    time::Duration,
};

static SCHEDULER_STORAGE: OnceLock<Arc<dyn BirthdayStorage>> = OnceLock::new();
static CACHE: OnceLock<Cache<String, bool>> = OnceLock::new();

pub async fn schedule_tasks(storage: Arc<dyn BirthdayStorage>) {
    if let Err(_) = SCHEDULER_STORAGE.set(storage) {
        panic!("Failed to set scheduler storage");
    }

    let mut scheduler = AsyncScheduler::new();
    scheduler.every(30.seconds()).run(|| {
        // I couldn't expect here, so I used unwrap
        // I don't know if this is the best way to do this
        // But it's the only way I could think of to get around the lifetime issues
        let thread_storage = SCHEDULER_STORAGE.get().unwrap().clone();
        async move {
            handle_reminders(thread_storage).await;
        }
    });

    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}

async fn send_birthday_reminder(name: &str, days_until_birthday: i64, age: Option<u32>) {
    let cache = CACHE.get_or_init(setup_cache);
    let cache_key = format!("{}-{}", name, days_until_birthday);
    if cache.get(&cache_key).is_some() {
        return;
    }

    let config = config::global();
    let http = discord::get_http_context();
    let message_channel = ChannelId(config.channel_id_to_post_reminders);

    let age = match age {
        Some(age) => format!("{} years old", age),
        None => "unknown age".to_string(),
    };

    let message = match days_until_birthday {
        0 => format!("It is {} birthday today!", name),
        1 => format!("It is {}'s birthday tomorrow!", name),
        3 => format!("In 3 days, {} has their birthday!", name),
        7 => format!("In one week, {} has their birthday!", name),
        _ => return,
    }
    .add(&format!(" They will be {}!", age));

    if let Err(why) = message_channel.say(http, message).await {
        println!("Failed to send birthday reminder for {}: {}", name, why);
    } else {
        cache.insert(cache_key, true);
    }
}

async fn handle_reminders(storage: Arc<dyn BirthdayStorage>) {
    let birthdays = match storage.get_birthdays().await {
        Ok(birthdays) => birthdays,
        Err(why) => {
            println!("Failed to get birthdays in scheduler, skipping: {}", why);
            return;
        }
    };

    for Birthday(name, date) in birthdays.iter() {
        if let Some(days_until_birthday) = days_until_next_occurrence(date) {
            send_birthday_reminder(
                name,
                days_until_birthday,
                calculate_age(*date, days_until_birthday),
            )
            .await;
        } else {
            println!("Failed to calculate days until birthday for {}", name);
        }
    }
}

fn days_until_next_occurrence(date: &NaiveDate) -> Option<i64> {
    let today = chrono::Local::now().naive_local().date();

    let date_this_year = match date.with_year(today.year()) {
        Some(date) => date,
        None => return None,
    };

    let date_next_year = match date.with_year(today.year() + 1) {
        Some(date) => date,
        None => return None,
    };

    let days_until_birthday = date_this_year.signed_duration_since(today).num_days();
    if days_until_birthday >= 0 {
        Some(days_until_birthday)
    } else {
        let days_until_birthday_next_year = date_next_year.signed_duration_since(today).num_days();
        Some(days_until_birthday_next_year)
    }
}

fn calculate_age(date_of_birth: NaiveDate, days_until_birthday: i64) -> Option<u32> {
    let days_until_birthday = chrono::Days::new(days_until_birthday as u64);
    let birthday = chrono::Local::now()
        .naive_local()
        .checked_add_days(days_until_birthday)
        .and_then(|birthday| Some(birthday.date()))?;

    birthday.years_since(date_of_birth)
}

fn setup_cache() -> Cache<String, bool> {
    Cache::builder()
        .time_to_live(Duration::from_secs(60 * 60 * 24))
        .build()
}
