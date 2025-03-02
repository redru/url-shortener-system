use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow)]
pub struct Url {
    pub id: i64,
    pub long_url: String,
    pub short_url: String,
}

pub struct ShortenerService {
    pool: PgPool,
}

impl ShortenerService {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Hardcoded for simplicity
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            std::env::var("POSTGRES_USER").unwrap_or_else(|_| "url_shortener".to_string()), // POSTGRES_USER
            std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "url_shortener".to_string()), // POSTGRES_PASSWORD
            std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()), // host
            std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5433".to_string()), // mapped port from docker-compose
            std::env::var("POSTGRES_DB").unwrap_or_else(|_| "url_shortener".to_string()), // POSTGRES_DB
        );

        // Create connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn find_by_short_url(&self, short_url: &str) -> Result<Option<Url>, sqlx::Error> {
        sqlx::query_as::<_, Url>("SELECT id, short_url, long_url FROM urls WHERE short_url = $1")
            .bind(short_url)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_by_long_url(&self, long_url: &str) -> Result<Option<Url>, sqlx::Error> {
        sqlx::query_as::<_, Url>("SELECT id, short_url, long_url FROM urls WHERE long_url = $1")
            .bind(long_url)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn insert_url(
        &self,
        id: i64,
        long_url: &str,
        short_url: &str,
    ) -> Result<Url, sqlx::Error> {
        let inserted_url = sqlx::query_as::<_, Url>(
            "INSERT INTO urls (id, long_url, short_url) VALUES ($1, $2, $3) RETURNING id, long_url, short_url"
        )
            .bind(id)
            .bind(long_url)
            .bind(short_url)
            .fetch_one(&self.pool)
            .await?;

        Ok(inserted_url)
    }
}
