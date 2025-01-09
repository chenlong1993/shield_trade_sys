// 存放控制器代码，用于提供 Web 接口和第三方 API 的集成。
pub mod base;
pub mod order;

use actix_web::web;

pub mod asset;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/asset")
                    .route(
                        "/deposit",
                        web::post().to(asset::UserAssetsController::deposit),
                    )
                    .route(
                        "/withdraw",
                        web::post().to(asset::UserAssetsController::withdraw),
                    )
                    .route(
                        "/transfer",
                        web::post().to(asset::UserAssetsController::transfer),
                    )
                    .route(
                        "/query",
                        web::get().to(asset::UserAssetsController::query_assets),
                    ),
            )
            // .service(
            //     web::scope("/order")
            //         .route(
            //             "/create",
            //             web::post().to(order::OrderController::create_order),
            //         )
            //         .route(
            //             "/history",
            //             web::get().to(order::OrderController::history_list),
            //         )
            //         .route(
            //             "/unfinished",
            //             web::get().to(order::OrderController::unfinished_list),
            //         ),
            // ),
    );
}
