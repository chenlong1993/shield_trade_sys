use actix_web::{web, App, HttpServer};
use shield_trade_sys::matching::MatchingEngine;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    
    // Initialize matching engine
    let matching_engine = web::Data::new(MatchingEngine::new());

    log::info!("Starting trading engine server on 127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(matching_engine.clone())
            .configure(shield_trade_sys::api::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
