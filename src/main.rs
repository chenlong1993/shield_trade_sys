use actix_web::{web, App, HttpServer};
use env_logger::Env;
use std::sync::{Arc, Mutex};
use crate::matching::MatchingEngine;
use crate::api::matching::config as matching_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Initialize matching engine
    let engine = Arc::new(Mutex::new(MatchingEngine::new()));

    log::info!("Starting trading engine server on 127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(engine.clone()))
            .configure(matching_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
