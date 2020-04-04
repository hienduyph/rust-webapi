use diesel;
use diesel::r2d2::ConnectionManager;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type MySQLPool = Pool<diesel::mysql::MysqlConnection>;
pub type SqlitePool = Pool<diesel::sqlite::SqliteConnection>;

#[cfg(feature = "mysql")]
pub type DBConn = MySQLPool;

#[cfg(feature = "sqlite")]
pub type DBConn = SqlitePool;

pub fn db_pool() -> DBConn {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<diesel::sqlite::SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
