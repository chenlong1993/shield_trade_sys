use dotenv::dotenv;
use std::env;
use sqlx::{Connection, PgConnection, Postgres, Error};
pub struct PostgreSQLConfig {
    pub database_url: String,
}

impl PostgreSQLConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("没有加载到配置文件");

        PostgreSQLConfig { database_url }
    }
}

pub async fn connect_database(config: &PostgreSQLConfig) -> Result<PgConnection, Error> {
    PgConnection::connect(&config.database_url).await.map_err(Error::from)
}