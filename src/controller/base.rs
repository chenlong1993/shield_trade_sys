use crate::common::response::Response;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

// 定义 BaseController 结构体
pub struct BaseController {
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
        data: Some("ok"),
        msg: "ok".parse().unwrap(),
        code: 200,
    });
    HttpResponse::Ok().json(data)
}
