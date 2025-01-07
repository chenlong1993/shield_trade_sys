use std::fmt;
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
    Buy,
    Sell,
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

pub type Numeric = BigDecimal;

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
