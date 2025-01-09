use crate::matching::{MatchingEngine, TradingEngine};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRequest {
    symbol: String,
    price: String,
    quantity: String,
    side: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/matching")
            .route("/order", web::post().to(place_order))
            .route("/order/{order_id}", web::delete().to(cancel_order))
            .route("/order/{order_id}", web::get().to(get_order))
            .route("/orders", web::get().to(get_orders))
            .route("/trades", web::get().to(get_trades))
            // .route("/orderbook/{symbol}", web::get().to(get_orderbook))
    );
}

async fn place_order(
    _engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    _order: web::Json<OrderRequest>,
) -> HttpResponse {
    todo!()
}

async fn cancel_order(
    _engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    _order_id: web::Path<Uuid>,
) -> HttpResponse {
      todo!()
}

async fn get_order(
    _engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    _order_id: web::Path<Uuid>,
) -> HttpResponse {
    todo!()
}

async fn get_orders(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
) -> HttpResponse {
    let engine = engine.lock().unwrap();
    HttpResponse::Ok().json(engine.get_orders().await)
}

async fn get_trades(
    _engine: web::Data<Arc<Mutex<MatchingEngine>>>,
) -> HttpResponse {
    todo!()
}

