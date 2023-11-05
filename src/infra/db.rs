#[cfg(feature = "postgres")]
pub type DBConn = sqlx::postgres::PgPool;

#[cfg(feature = "sqlite")]
pub type DBConn = sqlx::sqlite::SqlitePool;

#[cfg(feature = "mysql")]
pub type DBConn = sqlx::mysql::MySqlPool;

pub async fn db_conn() -> DBConn {
    let database_url = std::env::var("DATABASE_URL").unwrap_or("/tmp/test_examples.db".to_string());
    println!("Using Database {}", database_url);
    #[cfg(feature = "sqlite")]
    {
        sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap()
    }

    #[cfg(feature = "postgres")]
    {
        sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap()
    }

    #[cfg(feature = "mysql")]
    {
        sqlx::mysql::MySqlConnectOptions::new()
            .connect(&database_url)
            .await
            .unwrap()
    }
}
