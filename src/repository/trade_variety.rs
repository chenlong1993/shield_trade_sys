use crate::common::status::Status;
use crate::repository::base::Base;
use crate::repository::variety::Variety;
use chrono::DateTime;
use rust_decimal::Decimal;
use sqlx::types::BigDecimal;
use std::os::unix::raw::time_t;
use std::time::SystemTime;

// 对应TradeVariety结构体
#[derive(Debug)]
pub struct TradeVariety {
    id: i32,
    symbol: String,
    name: String,
    target_id: i32,
    base_id: i32,
    price_decimals: i32,
    qty_decimals: i32,
    allow_min_qty: BigDecimal,
    allow_max_qty: BigDecimal,
    allow_min_amount: BigDecimal,
    allow_max_amount: BigDecimal,
    pub(crate) fee_rate: BigDecimal,
    status: Status,
    sort: i64,
    base: Base,
    created_at: SystemTime,
    update_at: SystemTime,
}

// 对应CreateTradeVariety结构体
#[derive(Debug)]
pub struct CreateTradeVariety {
    symbol: String,
    name: String,
    target_id: i32,
    base_id: i32,
    price_decimals: Decimal,
    qty_decimals: Decimal,
    allow_min_qty: Decimal,
    allow_max_qty: Decimal,
    allow_min_amount: Decimal,
    allow_max_amount: Decimal,
    fee_rate: Decimal,
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
    price_decimals: Decimal,
    qty_decimals: Decimal,
    allow_min_qty: Decimal,
    allow_max_qty: Decimal,
    allow_min_amount: Decimal,
    allow_max_amount: Decimal,
    fee_rate: Decimal,
    status: Option<Status>,
    sort: Option<i64>,
}
