use crate::common::status::Status;
use crate::repository::base::Base;
use crate::repository::variety::Variety;

// 对应TradeVariety结构体
#[derive(Debug)]
pub struct TradeVariety {
    id: i32,
    symbol: String,
    name: String,
    target_id: i32,
    base_id: i32,
    target_variety: Option<Variety>,
    base_variety: Option<Variety>,
    price_decimals: i32,
    qty_decimals: i32,
    allow_min_qty: String,
    allow_max_qty: String,
    allow_min_amount: String,
    allow_max_amount: String,
    fee_rate: String,
    status: Status,
    sort: i64,
    base: Base,
}

// 对应CreateTradeVariety结构体
#[derive(Debug)]
pub struct CreateTradeVariety {
    symbol: String,
    name: String,
    target_id: i32,
    base_id: i32,
    price_decimals: i32,
    qty_decimals: i32,
    allow_min_qty: String,
    allow_max_qty: String,
    allow_min_amount: String,
    allow_max_amount: String,
    fee_rate: String,
    status: Status,
    sort: i64,
}

// 对应UpdateTradeVariety结构体
#[derive(Debug)]
pub struct UpdateTradeVariety {
    id: i32,
    symbol: Option<String>,
    name: Option<String>,
    target_id: Option<i32>,
    base_id: Option<i32>,
    price_decimals: Option<i32>,
    qty_decimals: Option<i32>,
    allow_min_qty: Option<String>,
    allow_max_qty: Option<String>,
    allow_min_amount: Option<String>,
    allow_max_amount: Option<String>,
    fee_rate: Option<String>,
    status: Option<Status>,
    sort: Option<i64>,
}

