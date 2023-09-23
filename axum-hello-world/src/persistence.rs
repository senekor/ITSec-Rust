use sqlx::{sqlite::SqlitePoolOptions, Error, Row, SqlitePool};

const IN_MEMORY_URL: &str = ":memory:";

pub struct SqliteInMemory {
    pub db: SqlitePool,
}

impl SqliteInMemory {
    pub async fn connect() -> Result<Self, Error> {
        let db = SqlitePoolOptions::new()
            .idle_timeout(None)
            .max_lifetime(None)
            .max_connections(1)
            .connect(IN_MEMORY_URL)
            .await?;

        Ok(SqliteInMemory { db })
    }

    pub async fn migrate(&self) -> Result<(), Error> {
        sqlx::migrate!("./migrations").run(&self.db).await?;
        Ok(())
    }

    pub async fn enter_text(&self, text: &str) -> Result<(), Error> {
        let mut transaction = self.db.begin().await?;

        sqlx::query("INSERT INTO texts (text) VALUES ($1)")
            .bind(text)
            .execute(&mut *transaction)
            .await?;
        transaction.commit().await?;

        Ok(())
    }

    pub async fn all_texts(&self) -> Result<Vec<String>, Error> {
        let texts = sqlx::query("SELECT text FROM texts")
            .fetch_all(&self.db)
            .await?
            .iter()
            .filter_map(|row| row.try_get(0).ok())
            .collect();

        Ok(texts)
    }
}
