pub mod base;
pub mod order;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(order::config)
    );
}
pub mod asset;
pub mod order;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/ping", web::get().to(base::BaseController::ping))
            .route("/time", web::get().to(base::BaseController::time))
            .route("/base/exchange_info", web::get().to(base::BaseController::exchange_info))
            .service(
                web::scope("/asset")
                    .route("/deposit", web::post().to(asset::UserAssetsController::deposit))
                    .route("/withdraw", web::post().to(asset::UserAssetsController::withdraw))
                    .route("/transfer", web::post().to(asset::UserAssetsController::transfer))
                    .route("/query", web::get().to(asset::UserAssetsController::query_assets))
            )
            .service(
                web::scope("/order")
                    .route("/create", web::post().to(order::OrderController::create_order))
                    .route("/history", web::get().to(order::OrderController::history_list))
                    .route("/unfinished", web::get().to(order::OrderController::unfinished_list))
            )
    );
}
