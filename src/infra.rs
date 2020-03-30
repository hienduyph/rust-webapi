use diesel::r2d2::ConnectionManager;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type MySQLPool = Pool<diesel::mysql::MysqlConnection>;

#[cfg(feature = "mysql")]
pub type DBConn = MySQLPool;
