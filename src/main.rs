use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::{Arc, Mutex};
use crate::matching::MatchingEngine;
use crate::api::matching::config as matching_config;
use crate::controller::order::OrderController;
use crate::repository::{PostgresOrderRepository, PostgresTradeVarietyRepository, InMemoryOrderRepository, InMemoryTradeVarietyRepository};
use sqlx::PgPool;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize matching engine
    let engine = Arc::new(Mutex::new(MatchingEngine::new()));

    // Configure logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Initialize database connection pool if needed
    let pool = if let Ok(database_url) = env::var("DATABASE_URL") {
        Some(PgPool::connect(&database_url).await
            .expect("Failed to connect to database"))
    } else {
        None
    };

    // Initialize repositories if database is configured
    let order_repo = pool.as_ref().map(|p| Arc::new(PostgresOrderRepository::new(p.clone())));
    let trade_variety_repo = pool.map(|p| Arc::new(PostgresTradeVarietyRepository::new(p)));

    // Initialize order controller
    let order_controller = Arc::new(OrderController::new(
        engine.clone(),
        Arc::new(env_logger::Logger::default()),
        order_repo.unwrap_or_else(|| Arc::new(InMemoryOrderRepository::new())),
        trade_variety_repo.unwrap_or_else(|| Arc::new(InMemoryTradeVarietyRepository::new())),
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
