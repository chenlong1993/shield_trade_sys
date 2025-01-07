use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::matching::MatchingEngine;
use std::sync::{Arc, Mutex};

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
            .route("/orderbook/{symbol}", web::get().to(get_orderbook))
    );
}

async fn place_order(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    order: web::Json<OrderRequest>,
) -> HttpResponse {
    let mut engine = engine.lock().unwrap();
    match engine.add_order(order.into_inner().into()).await {
        Ok(trades) => HttpResponse::Ok().json(trades),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

async fn cancel_order(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    order_id: web::Path<Uuid>,
) -> HttpResponse {
    let mut engine = engine.lock().unwrap();
    match engine.cancel_order("", *order_id).await {
        Ok(_) => HttpResponse::Ok().json("Order cancelled successfully"),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

async fn get_order(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    order_id: web::Path<Uuid>,
) -> HttpResponse {
    let engine = engine.lock().unwrap();
    match engine.get_order(*order_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}

async fn get_orders(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
) -> HttpResponse {
    let engine = engine.lock().unwrap();
    HttpResponse::Ok().json(engine.get_orders().await)
}

async fn get_trades(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
) -> HttpResponse {
    let engine = engine.lock().unwrap();
    HttpResponse::Ok().json(engine.get_trades().await)
}

async fn get_orderbook(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    symbol: web::Path<String>,
) -> HttpResponse {
    let engine = engine.lock().unwrap();
    match engine.get_orderbook(&symbol).await {
        Ok(orderbook) => HttpResponse::Ok().json(orderbook),
        Err(e) => HttpResponse::NotFound().json(e.to_string()),
    }
}
