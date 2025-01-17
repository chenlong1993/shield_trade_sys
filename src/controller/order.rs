use crate::common::response::Response;
use crate::domain::request::order_create_request::OrderCreateRequest;
use crate::service::order;
use actix_web::{post, web, HttpResponse, Responder};
use rust_decimal::Decimal;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    symbol: String,
    price: Option<Decimal>, // 限价单需要价格
    quantity: Decimal,
    side: String,
    amount: Option<Decimal>, // 市价单按金额时需要
}

#[post("/create_order")]
async fn create_order(
    db: web::Data<DatabaseConnection>,
    req: web::Json<OrderCreateRequest>,
) -> impl Responder {
    match order::create_order(db, req).await {
        Ok(user) => HttpResponse::Ok().json(Response::success(Some(user))),
        Err(e) => HttpResponse::InternalServerError().json(Response::failed(e.to_string())),
    }
}

    //
    // /**
    //  * cancel order
    //  * 取消订单
    //  */
    // pub async fn cancel_order(
    //     engine: web::Data<Arc<Mutex<MatchingEngine>>>,
    //     order_repo: web::Data<Box<dyn OrderRepository>>,
    //     order_id: web::Path<Uuid>,
    // ) -> impl Responder {
    //     match order_repo.cancel_order("", &order_id.to_string(), "user").await {
    //         Ok(_) => {
    //             // 从撮合引擎移除订单
    //             match engine.lock().unwrap().cancel_order("", *order_id).await {
    //                 Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Order canceled")),
    //                 Err(e) => HttpResponse::BadRequest().json(ApiResponse::error(400, e.to_string())),
    //             }
    //         },
    //         Err(e) => match e {
    //             OrderError::ValidationError(msg) => {
    //                 HttpResponse::BadRequest().json(ApiResponse::error(400, msg))
    //             },
    //             OrderError::DatabaseError(msg) => {
    //                 HttpResponse::InternalServerError().json(ApiResponse::error(500, msg))
    //             },
    //             OrderError::AssetError(msg) => {
    //                 HttpResponse::BadRequest().json(ApiResponse::error(400, msg))
    //             },
    //             OrderError::NotFound => {
    //                 HttpResponse::NotFound().json(ApiResponse::error(404, "Order not found".to_string()))
    //             },
    //         },
    //     }
    // }

    // pub fn config(cfg: &mut web::ServiceConfig) {
    //     cfg.service(
    //         web::scope("/orders")
    //             .route("", web::post().to(create_order))
    //             .route("/{order_id}", web::delete().to(cancel_order)),
    //     );
    // }
