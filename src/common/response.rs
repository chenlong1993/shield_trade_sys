use actix_web::web;
use rust_decimal::prelude::FromStr;
use rust_decimal::Decimal;
use serde::Serialize;

// 响应结构体
#[derive(Serialize)]
pub struct Response<T> {
    pub(crate) data: Option<T>,
    pub(crate) code: i32,
    pub(crate) msg:String
}

// 成功响应
impl <T> Response<T>{
    pub fn success(data: Option<T>) -> web::Json<Response<T>> {
        web::Json(Response {data,code: 200,msg: String::from("操作成功"),})
    }
    pub fn failed(msg: String) -> web::Json<Response<String>> {
        web::Json(Response { data:None, code: 200, msg})
    }
}

// 格式化字符串数字
fn format_str_number(n: &str, p: u32) -> Result<String, rust_decimal::Error> {
    let d = Decimal::from_str(n)?;
    Ok(d.round_dp(p).to_string())
}

