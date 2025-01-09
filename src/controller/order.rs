use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    symbol: String,
    price: Option<Decimal>, // 限价单需要价格
    quantity: Decimal,
    side: String,
    amount: Option<Decimal>, // 市价单按金额时需要
}
//
// /**
//  * Create a new order
//  * 创建一个新的订单
//   */
// pub async fn create_order(
//     engine: web::Data<Arc<Mutex<MatchingEngine>>>,
//     order_repo: web::Data<Box<dyn OrderRepository>>,
//     req: web::Json<CreateOrderRequest>
// ) -> impl Responder {
//     // 验证订单类型
//     let order_type = match req.price {
//         Some(_) => "limit",
//         None => "market",
//     };
//
//     // 创建订单
//     let result = match order_type {
//         "limit" => {
//             if let Some(price) = req.price {
//                 order_repo.create_limit_order(
//                     "user_id", // TODO: 从认证信息获取
//                     &req.symbol,
//                     &req.side,
//                     price,
//                     req.quantity,
//                 ).await
//             } else {
//                 return HttpResponse::BadRequest().json(
//                     ApiResponse::error(400, "Price is required for limit orders".to_string())
//                 );
//             }
//         },
//         "market" => {
//             if let Some(amount) = req.amount {
//                 order_repo.create_market_order_by_amount(
//                     "user_id", // TODO: 从认证信息获取
//                     &req.symbol,
//                     &req.side,
//                     amount,
//                 ).await
//             } else {
//                 order_repo.create_market_order_by_quantity(
//                     "user_id", // TODO: 从认证信息获取
//                     &req.symbol,
//                     &req.side,
//                     req.quantity,
//                 ).await
//             }
//         },
//         _ => {
//             return HttpResponse::BadRequest().json(
//                 ApiResponse::error(400, "Invalid order type".to_string())
//             );
//         }
//     };
//
//     match result {
//         Ok(order) => {
//             // 将订单添加到撮合引擎
//             match engine.lock().unwrap().add_order(order).await {
//                 Ok(trades) => HttpResponse::Ok().json(ApiResponse::success(trades)),
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
