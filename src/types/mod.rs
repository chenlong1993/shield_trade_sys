use std::fmt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub symbol: String,
    pub price: Numeric,
    pub quantity: Numeric,
    pub side: OrderSide,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,//购买
    Sell,//出售
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: Uuid,
    pub symbol: String,
    pub price: Numeric,
    pub quantity: Numeric,
    pub taker_side: OrderSide,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub symbol: String,
    pub price: String,
    pub quantity: String,
    pub side: String,
}

#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    // 新建  但未被提交到市场
    New,
    // 等待触发  等待触发
    Pending,
    // 已提交  已提交市场，等待执行
    Submitted,
    // 部分成交  部分成交，但尚未完全执行
    PartialFill,
    // 已成交  订单已经完全执行，所有股票或合约已经被买入或卖出
    Filled,
    // 已过期  如果订单设置了有效期，且在规定时间内未能成交，订单可能会被标记为已过期
    Expired,
    // 已拒绝  交易所或经纪商可能会拒绝执行某些类型的订单，例如超过限制的市价单
    Rejected,
    // 部分取消   在部分成交后，交易者可能取消尚未执行的部分订单
    PartialCancel,
    // 已取消  交易者或系统取消了订单，订单不再有效
    Canceled,
}

#[derive(Debug, PartialEq)]
pub enum CancelType {
    // 用户主动取消
    User,
    // 系统取消
    System,
    // 超时取消
    Expired,
    // 市场取消
    Market,
    // 强平取消
    Force,
    // 其他
    Other,
}

pub type Numeric = Decimal;

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "Buy"),
            OrderSide::Sell => write!(f, "Sell"),
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order {{ id: {}, symbol: {}, price: {}, quantity: {}, side: {}, timestamp: {} }}",
            self.id, self.symbol, self.price, self.quantity, self.side, self.timestamp
        )
    }
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trade {{ id: {}, symbol: {}, price: {}, quantity: {}, taker_side: {}, timestamp: {} }}",
            self.id, self.symbol, self.price, self.quantity, self.taker_side, self.timestamp
        )
    }
}
