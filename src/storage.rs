use chrono::NaiveDate;
use serenity::async_trait;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{error::Error, sync::Arc};

#[derive(Debug)]
pub struct Birthday(pub String, pub NaiveDate);

#[async_trait]
pub trait Storage: Send + Sync {
    async fn add_birthday(&self, name: &str, date: NaiveDate) -> Result<(), Box<dyn Error>>;
    async fn remove_birthday(&self, name: &str) -> Result<(), Box<dyn Error>>;
    async fn get_birthdays(&self) -> Result<Vec<Birthday>, Box<dyn Error>>;
}

pub struct SqliteStorage {
    conn: Pool<Sqlite>,
}

impl SqliteStorage {
    pub async fn new() -> Result<SqliteStorage, Box<dyn Error>> {
        let database_url = dotenvy::var("DATABASE_URL")?;
        let conn = SqlitePoolOptions::new().connect(&database_url).await?;
        Ok(SqliteStorage { conn })
    }

    pub async fn arc() -> Result<Arc<SqliteStorage>, Box<dyn Error>> {
        let storage = Self::new().await?;
        Ok(Arc::new(storage))
    }
}

#[async_trait]
impl Storage for SqliteStorage {
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
