use actix_web::{web, App, HttpServer};
use env_logger::Env;
use shield_trade_sys::MatchingEngine;
use std::env;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;

mod infra;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_config = infra::postgresql::PostgreSQLConfig::from_env();
    let redis_config = infra::redis::RedisConfig::from_env();
    let _db = infra::postgresql::connect_database(&db_config).await;
    let _redis = infra::redis::connect_redis(&redis_config);
    // Initialize matching engine
    let engine = Arc::new(Mutex::new(MatchingEngine::new()));

    // Configure logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("Starting trading engine server on 127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(engine.clone()))
            // .configure(matching_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
