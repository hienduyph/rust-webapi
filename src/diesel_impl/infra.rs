use diesel;
use diesel::r2d2::ConnectionManager;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type MySQLPool = Pool<diesel::mysql::MysqlConnection>;
pub type SqlitePool = Pool<diesel::sqlite::SqliteConnection>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;

#[cfg(feature = "mysql")]
pub type DBConn = MySQLPool;
#[cfg(feature = "mysql")]
pub type Conn = diesel::mysql::MysqlConnection;

#[cfg(feature = "sqlite")]
pub type DBConn = SqlitePool;
#[cfg(feature = "sqlite")]
pub type Conn = diesel::sqlite::SqliteConnection;

#[cfg(feature = "postgres")]
pub type DBConn = PostgresPool;
#[cfg(feature = "postgres")]
pub type Conn = diesel::pg::PgConnection;

pub fn db_pool() -> DBConn {
    let database_url = std::env::var("DATABASE_URL").unwrap_or("/tmp/test_examples.db".to_string());
    println!("Using Database {}", database_url);
    let manager = ConnectionManager::<Conn>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
