use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// 定义 BaseController 结构体
pub struct BaseController {
    trade_variety: Box<dyn TradeVarietyRepository>,
    state: ()
}
impl BaseController {
    // ping 处理函数
    pub(crate) async fn ping(&self) -> impl Responder {
        HttpResponse::Ok().json("pong")
    }

    // time 处理函数
    pub(crate) async fn time(&self) -> impl Responder {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos();
        HttpResponse::Ok().json(serde_json::json!({ "time": now }))
    }

    // exchange_info 处理函数
    pub(crate) async fn exchange_info(&self, req: HttpRequest) -> impl Responder {
        let symbol = req
            .query_string()
            .split('=')
            .nth(1)
            .unwrap_or("")
            .to_uppercase();

        let state = self.state.lock().unwrap();
        match state.trade_variety.find_by_symbol(&symbol) {
            Some(trade_variety) => HttpResponse::Ok().json(trade_variety),
            None => HttpResponse::NotFound().json("Trade variety not found"),
        }
    }
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse::success("ok"))
}
