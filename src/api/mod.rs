pub mod matching;

use actix_web::{web, App, HttpServer};
use crate::matching::MatchingEngine;
use std::sync::{Arc, Mutex};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(matching::config())
    );
}

pub fn create_engine() -> Arc<Mutex<MatchingEngine>> {
    Arc::new(Mutex::new(MatchingEngine::new()))
}

pub async fn start_server(engine: Arc<Mutex<MatchingEngine>>) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(engine.clone()))
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
