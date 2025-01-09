use actix_web::{HttpResponse, Responder};
use rust_decimal::prelude::FromStr;
use rust_decimal::Decimal;
use serde::Serialize;

// 响应结构体
#[derive(Serialize)]
pub struct Response<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub msg: Option<String>,
    pub code: Option<i8>,
}

// 成功响应
pub async fn response_ok<T: Serialize>(data: T) -> impl Responder {
    let response = Response {
        ok: true,
        data: Some(data),
        code: None,
        msg: None,
    };
    HttpResponse::Ok().json(response)
}

// 错误响应
async fn response_error(err: &str) -> impl Responder {
    let response = Response::<()> {
        ok: false,
        data: None,
        msg: Some(err.to_string()),
        code: None,
    };
    HttpResponse::Ok().json(response)
}

// 格式化字符串数字
fn format_str_number(n: &str, p: u32) -> Result<String, rust_decimal::Error> {
    let d = Decimal::from_str(n)?;
    Ok(d.round_dp(p).to_string())
}

