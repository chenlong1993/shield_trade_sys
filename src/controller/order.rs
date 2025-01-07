use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::TradingError;
use crate::types::Order;
use crate::matching::MatchingEngine;
use std::sync::{Arc, Mutex};
use super::base::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    symbol: String,
    price: f64,
    quantity: f64,
    side: String,
}

pub async fn create_order(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    req: web::Json<CreateOrderRequest>,
) -> impl Responder {
    let order = Order::new(
        req.symbol.clone(),
        req.price,
        req.quantity,
        req.side.clone(),
    );

    match engine.lock().unwrap().add_order(order).await {
        Ok(trades) => HttpResponse::Ok().json(ApiResponse::success(trades)),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::error(400, e.to_string())),
    }
}

pub async fn cancel_order(
    engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    order_id: web::Path<Uuid>,
) -> impl Responder {
    match engine.lock().unwrap().cancel_order("", *order_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Order canceled")),
        Err(e) => HttpResponse::BadRequest().json(ApiResponse::error(400, e.to_string())),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .route("", web::post().to(create_order))
            .route("/{order_id}", web::delete().to(cancel_order))
    );
}
