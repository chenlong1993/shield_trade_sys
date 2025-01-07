use actix_web::{web, HttpResponse, Responder};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;
use serde::Serialize;

// 响应结构体
#[derive(Serialize)]
struct Response<T> {
    ok: bool,
    data: Option<T>,
    msg: Option<String>,
}

// 成功响应
async fn response_ok<T: Serialize>(data: T) -> impl Responder {
    let response = Response {
        ok: true,
        data: Some(data),
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
    };
    HttpResponse::Ok().json(response)
}

// 格式化字符串数字
fn format_str_number(n: &str, p: u32) -> Result<String, rust_decimal::Error> {
    let d = Decimal::from_str(n)?;
    Ok(d.round_dp(p).to_string())
}

// 示例路由
async fn example_route() -> impl Responder {
    match format_str_number("1234.56789", 2) {
        Ok(result) => response_ok(result).await,
        Err(err) => response_error(&err.to_string()).await,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(example_route))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}