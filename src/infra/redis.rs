use dotenv::dotenv;
use redis::Client as RedisClient;
use std::env;

pub struct RedisConfig {
    pub redis_url: String,
}

impl RedisConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

        RedisConfig { redis_url }
    }
}

pub fn connect_redis(config: &RedisConfig) -> RedisClient {
    RedisClient::open(config.redis_url.as_str()).expect("Failed to connect to Redis")
}