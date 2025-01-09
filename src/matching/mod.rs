//匹配订单，撮合订单
pub mod order_book;
pub mod matching_engine;

use crate::error::TradingResult;
use crate::types::{Order, Trade};
use async_trait::async_trait;

#[async_trait]
pub trait TradingEngine {
    /// Add a new order to the engine
    async fn add_order(&mut self, order: Order) -> TradingResult<()>;
    
    /// Cancel an existing order
    // async fn cancel_order(&mut self, order_id: &uuid::Uuid) -> TradingResult<Order>;
    
    /// Get order by ID
    async fn get_order(&self, order_id: &uuid::Uuid) -> TradingResult<&Order>;
    
    /// Get all active orders
    async fn get_orders(&self) -> TradingResult<Vec<&Order>>;
    
    /// Get trades for a specific order
    async fn get_order_trades(&self, order_id: &uuid::Uuid) -> TradingResult<Vec<&Trade>>;
    
    /// Get all trades
    async fn get_trades(&self) -> TradingResult<Vec<&Trade>>;
    
    // Get order book depth
    // async fn get_order_book(&self, depth: usize) -> TradingResult<Vec<(Numeric, Numeric)>>;
    // async fn get_orderbook(&self, symbol: &str) -> Result<OrderBookSnapshot, TradingError>;
}

pub use matching_engine::MatchingEngine;
pub use order_book::OrderBook;


