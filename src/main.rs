use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::{Arc, Mutex};
use crate::matching::MatchingEngine;
use crate::api::matching::config as matching_config;
use crate::controller::order::OrderController;
use crate::repository::{PostgresOrderRepository, PostgresTradeVarietyRepository};
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Initialize database connection pool
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await
        .expect("Failed to connect to database");

    // Initialize matching engine
    let engine = Arc::new(Mutex::new(MatchingEngine::new()));

    // Initialize repositories
    let order_repo = Arc::new(PostgresOrderRepository::new(pool.clone()));
    let trade_variety_repo = Arc::new(PostgresTradeVarietyRepository::new(pool));

    // Initialize order controller
    let order_controller = Arc::new(OrderController::new(
        engine.clone(),
        Arc::new(env_logger::Logger::default()),
        order_repo,
        trade_variety_repo,
    ));

    log::info!("Starting trading engine server on 127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(engine.clone()))
            .app_data(web::Data::new(order_controller.clone()))
            .configure(matching_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
