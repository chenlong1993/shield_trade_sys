use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;
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

pub async fn connect_database(config: &PostgreSQLConfig) -> DatabaseConnection {
    Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to the database")
}
