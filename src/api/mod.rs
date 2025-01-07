use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::{Order, OrderSide, OrderType};
use crate::matching::MatchingEngine;
use crate::error::TradingResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub price: String,
    pub quantity: String,
    pub side: OrderSide,
    pub order_type: OrderType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    pub order_id: Uuid,
}

pub async fn create_order(
    data: web::Data<MatchingEngine>,
    req: web::Json<CreateOrderRequest>,
) -> impl Responder {
    let order = Order::new(
        &req.symbol,
        req.price.parse().unwrap(),
        req.quantity.parse().unwrap(),
        req.side,
        req.order_type,
        None,
    );

    match data.add_order(order).await {
        Ok(_) => HttpResponse::Ok().json("Order created successfully"),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub async fn cancel_order(
    data: web::Data<MatchingEngine>,
    req: web::Json<CancelOrderRequest>,
) -> impl Responder {
    match data.cancel_order(&req.order_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub async fn get_order(
    data: web::Data<MatchingEngine>,
    order_id: web::Path<Uuid>,
) -> impl Responder {
    match data.get_order(&order_id).await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub async fn get_orders(data: web::Data<MatchingEngine>) -> impl Responder {
    match data.get_orders().await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub async fn get_trades(data: web::Data<MatchingEngine>) -> impl Responder {
    match data.get_trades().await {
        Ok(trades) => HttpResponse::Ok().json(trades),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub async fn get_order_book(
    data: web::Data<MatchingEngine>,
    depth: web::Path<usize>,
) -> impl Responder {
    match data.get_order_book(*depth).await {
        Ok(depth) => HttpResponse::Ok().json(depth),
        Err(e) => HttpResponse::BadRequest().json(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/order", web::post().to(create_order))
            .route("/order/{order_id}", web::delete().to(cancel_order))
            .route("/order/{order_id}", web::get().to(get_order))
            .route("/orders", web::get().to(get_orders))
            .route("/trades", web::get().to(get_trades))
            .route("/orderbook/{depth}", web::get().to(get_order_book)),
    );
}
