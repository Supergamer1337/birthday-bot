use crate::config;
use chrono::NaiveDate;
use serenity::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::{error::Error, sync::Arc};

#[derive(Debug)]
pub struct Birthday(pub String, pub NaiveDate);

#[async_trait]
pub trait BirthdayStorage: Send + Sync {
    async fn add_birthday(&self, name: &str, date: NaiveDate) -> Result<(), Box<dyn Error>>;
    async fn remove_birthday(&self, name: &str) -> Result<(), Box<dyn Error>>;
    async fn get_birthdays(&self) -> Result<Vec<Birthday>, Box<dyn Error>>;
}

pub struct SqliteStorage {
    conn: Pool<Sqlite>,
}

impl SqliteStorage {
    pub async fn new() -> Result<SqliteStorage, Box<dyn Error>> {
        let config = config::global();
        let options = SqliteConnectOptions::new()
            .filename(format!("./data/{}", config.db_name))
            .create_if_missing(true);
        let conn = SqlitePoolOptions::new().connect_with(options).await?;

        sqlx::migrate!("./migrations").run(&conn).await?;

        Ok(SqliteStorage { conn })
    }

    pub async fn arc() -> Result<Arc<SqliteStorage>, Box<dyn Error>> {
        let storage = Self::new().await?;
        Ok(Arc::new(storage))
    }
}

#[async_trait]
impl BirthdayStorage for SqliteStorage {
    async fn add_birthday(&self, name: &str, date: NaiveDate) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            "INSERT INTO birthdays (name, birthday) VALUES (?, ?)",
            name,
            date
        )
        .execute(&self.conn)
        .await?;
        Ok(())
    }

    async fn remove_birthday(&self, name: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query!("DELETE FROM birthdays WHERE name = ?", name)
            .execute(&self.conn)
            .await?;
        Ok(())
    }

    async fn get_birthdays(&self) -> Result<Vec<Birthday>, Box<dyn Error>> {
        let birthdays = sqlx::query!("SELECT name, birthday FROM birthdays")
            .fetch_all(&self.conn)
            .await?
            .into_iter()
            .map(|b| Birthday(b.name, b.birthday))
            .collect::<Vec<_>>();

        Ok(birthdays)
    }
}
