use crate::common::response::Response;
use crate::repository::trade_variety::TradeVariety;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

// 定义 BaseController 结构体
pub struct BaseController {
    trade_variety: Box<TradeVariety>,
    state: (),
}
impl BaseController {

    // exchange_info 处理函数
    pub(crate) async fn exchange_info(&self, _req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
        todo!()
    }
}

//健康检查
pub async fn health_check() -> impl Responder {
    let data = web::Data::new(Response {
        ok: true,
        data: Some("ok"),
        msg: Option::from("ok".to_string()),
        code: Some(20),
    });
    HttpResponse::Ok().json(data)
}
