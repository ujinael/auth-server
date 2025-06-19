use bb8_redis::{bb8::Pool, RedisConnectionManager};
use dotenvy::dotenv;
use std::env;

pub type RedisPool = Pool<RedisConnectionManager>;
pub async fn init_redis() -> RedisPool {
    dotenv().ok();
    let database_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let manager = RedisConnectionManager::new(database_url).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();
    // .expect("Error when createing redis pool");
    return pool;
}
