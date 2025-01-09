use actix_web::{web, App, HttpServer};
use env_logger::Env;
use shield_trade_sys::MatchingEngine;
use std::env;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
