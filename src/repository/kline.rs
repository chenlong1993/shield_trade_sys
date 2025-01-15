use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Kline {
    pub uuid: Uuid,                 // 唯一标识符
    pub base: String,               // 基础字段，根据实际情况调整类型
    pub symbol: String,             // 交易对符号
    pub period: String,             // 时间周期类型
    pub open_at: NaiveDateTime,     // 开始时间
    pub close_at: NaiveDateTime,    // 结束时间
    pub open: String,               // 开盘价
    pub high: String,               // 最高价
    pub low: String,                // 最低价
    pub close: String,              // 收盘价
    pub volume: String,             // 交易量
    pub amount: String,             // 交易金额
}