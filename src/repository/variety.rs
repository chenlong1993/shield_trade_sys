use crate::common::status::Status;
use crate::repository::base::Base;

// 对应Variety结构体
#[derive(Debug)]
pub struct Variety {
    id: i32,
    symbol: String,
    name: String,
    show_decimals: i32,
    min_decimals: i32,
    is_base: bool,
    sort: i64,
    status: Status,
    base: Base,
}

// 对应CreateVariety结构体
#[derive(Debug)]
pub struct CreateVariety {
    symbol: String,
    name: String,
    show_decimals: i32,
    min_decimals: i32,
    is_base: bool,
    sort: i64,
    status: Status,
}

// 对应UpdateVariety结构体
#[derive(Debug)]
pub struct UpdateVariety {
    id: i32,
    symbol: Option<String>,
    name: Option<String>,
    show_decimals: Option<i32>,
    min_decimals: Option<i32>,
    is_base: Option<bool>,
    sort: Option<i64>,
    status: Option<Status>,
}
