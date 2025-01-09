//用于与第三方APP交互
pub mod matching;

use crate::matching::MatchingEngine;
use actix_web::{web, App, HttpServer};
use std::sync::{Arc, Mutex};

pub fn config(_cfg: &mut web::ServiceConfig) {
    todo!()
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
