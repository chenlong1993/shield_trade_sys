use std::str::FromStr;
use actix_web::{post, web, HttpResponse, Responder};
use log::error;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::domain::request::order_create_request::OrderCreateRequest;
use crate::models::event_order_new::EventOrderNew;
use crate::repository::base::UUID;
use crate::repository::order::Order;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    symbol: String,
    price: Option<Decimal>, // 限价单需要价格
    quantity: Decimal,
    side: String,
    amount: Option<Decimal>, // 市价单按金额时需要
}

#[post("/orders")]
async fn create_order(req: web::Json<OrderCreateRequest>, user_id: web::ReqData<UUID>) -> impl Responder {
    let req = req.into_inner();
    let user_id = user_id.into_inner().id;
    //如果是限价单
    if req.order_type == "limit" {
        if req.price.is_none() || req.quantity.is_none() {
            return HttpResponse::BadRequest().json("price and quantity are required");
        }

        // 创建限价订单逻辑
        let price = Decimal::from_str(&req.price.unwrap()).unwrap();
        let quantity = Decimal::from_str(&req.quantity.unwrap()).unwrap();

        // 假设 create_limit_order 返回订单和错误
        match create_limit_order(&*user_id, &req.symbol, &req.side, price, quantity).await {
            Ok(order) => {
                let event = EventOrderNew {
                    symbol: Some(order.symbol.clone()),
                    order_id: Some(order.order_id.clone()),
                    order_side: Some(order.order_side.clone()),
                    order_type: Some(order.order_type.clone()),
                    nano_time: Some(order.nano_time.clone()),
                    price: Some(price),
                    quantity: Some(quantity),
                    amount: None,
                    max_amount: None,
                    max_qty: None,
                };

                publish_event(event).await;

                HttpResponse::Ok().json(order.order_id)
            }
            Err(err) => {
                error!("create limit order error: {:?}", err);
                HttpResponse::InternalServerError().json("create limit order error")
            }
        }
    } else {
        if req.amount.is_none() && req.quantity.is_none() {
            return HttpResponse::BadRequest().json("amount or quantity is required");
        }

        // 创建市价订单逻辑
        if let Some(amount) = req.amount {
            let amount = Decimal::from_str(&amount).unwrap();

            match create_market_order_by_amount(&*user_id, &req.symbol, &req.side, amount).await {
                Ok(order) => {
                    let event = EventOrderNew {
                        symbol: Some(order.symbol.clone()),
                        order_id: Some(order.order_id.clone()),
                        order_side: Some(order.order_side.clone()),
                        order_type: Some(order.order_type.clone()),
                        nano_time: Some(order.nano_time),
                        price: None,
                        quantity: None,
                        amount: Some(amount),
                        max_amount: Some(order.freeze_amount.clone()),
                        max_qty: None,
                    };

                    publish_event(event).await;

                    HttpResponse::Ok().json(order.order_id)
                }
                Err(err) => {
                    error!("create market order by amount error: {:?}", err);
                    HttpResponse::InternalServerError().json("create market order by amount error")
                }
            }
        } else {
            let quantity = Decimal::from_str(&req.quantity.unwrap()).unwrap();

            match create_market_order_by_qty(&*user_id, &req.symbol, &req.side, quantity).await {
                Ok(order) => {
                    let event = EventOrderNew {
                        symbol: Some(order.symbol.clone()),
                        order_id:  Some(order.order_id.clone()),
                        order_side: Some(order.order_side.clone()),
                        order_type: Some(order.order_type.clone()),
                        nano_time: Some(order.nano_time),
                        price: None,
                        quantity: Some(quantity),
                        amount: None,
                        max_amount: None,
                        max_qty: Some(order.freeze_qty),
                    };

                    publish_event(event).await;

                    HttpResponse::Ok().json(order.order_id)
                }
                Err(err) => {
                    error!("create market order by qty error: {:?}", err);
                    HttpResponse::InternalServerError().json("create market order by qty error")
                }
            }
        }
    }
}

// 假设的数据库操作和事件发布函数
async fn create_limit_order(user_id: &str, symbol: &str, side: &str, price: Decimal, quantity: Decimal) -> Result<Order, String> {
    // 实现创建限价订单的逻辑
    unimplemented!()
}
async fn create_market_order_by_amount(user_id: &str, symbol: &str, side: &str, amount: Decimal) -> Result<Order, String> {
    // 实现创建按金额市价订单的逻辑
    unimplemented!()
}

async fn create_market_order_by_qty(user_id: &str, symbol: &str, side: &str, quantity: Decimal) -> Result<Order, String> {
    // 实现创建按数量市价订单的逻辑
    unimplemented!()
}

async fn publish_event(event: EventOrderNew) {
    // 实现发布事件的逻辑
    unimplemented!()
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
