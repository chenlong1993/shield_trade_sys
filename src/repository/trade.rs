use serde::{Deserialize, Serialize};
//交易日志结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeLog {
    pub uuid: String,          // 唯一标识符，用于标识每一个交易日志
    pub base: String,          // 基础字段，具体用途需要根据业务逻辑确定
    #[serde(skip_serializing)]
    pub symbol: String,        // 交易对的符号，例如 "BTC_USD"，序列化时忽略
    pub trade_id: String,      // 交易ID，每笔交易的唯一标识符
    pub ask: String,           // 卖方订单ID
    pub bid: String,           // 买方订单ID
    pub trade_by: i16,         // 交易方式，由 matching_types.TradeBy 表示，具体数值含义需要参考 matching_types 模块
    pub ask_uid: String,       // 卖方用户ID
    pub bid_uid: String,       // 买方用户ID
    pub price: String,         // 成交价格
    pub quantity: String,      // 成交数量
    pub amount: String,        // 成交金额
    pub ask_fee_rate: String,  // 卖方手续费率
    pub ask_fee: String,       // 卖方手续费
    pub bid_fee_rate: String,  // 买方手续费率
    pub bid_fee: String,       // 买方手续费
}

impl TradeLog {
    pub fn table_name(&self) -> String {
        format!("trade_log_{}", self.symbol)
    }
}