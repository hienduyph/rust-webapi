use deadpool_redis::redis::cmd;
use deadpool_redis::{Config, Manager, Pool, Runtime};

type RedisPool = Pool;

pub async fn redis_client() -> RedisPool {
    let default = "redis://127.0.0.1:6379".to_string();
    let pool_cfg = Config::from_url(default);
    pool_cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}

pub async fn redis_sample() {
    let pool = redis_client().await;
    {
        let mut conn = pool.get().await.unwrap();
        cmd("SET")
            .arg(&["deadpool/test_key", "42"])
            .query_async::<_, ()>(&mut conn)
            .await
            .unwrap();
    }
    {
        let mut conn = pool.get().await.unwrap();
        let value: String = cmd("GET")
            .arg(&["deadpool/test_key"])
            .query_async(&mut conn)
            .await
            .unwrap();
        assert_eq!(value, "42".to_string());
    }
}
