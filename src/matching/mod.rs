pub mod order_book;
pub mod matching_engine;

use async_trait::async_trait;
use crate::types::{Numeric, Order, Trade};
use crate::error::TradingResult;

#[async_trait]
pub trait TradingEngine {
    /// Add a new order to the engine
    async fn add_order(&mut self, order: Order) -> TradingResult<()>;
    
    /// Cancel an existing order
    async fn cancel_order(&mut self, order_id: &uuid::Uuid) -> TradingResult<Order>;
    
    /// Get order by ID
    async fn get_order(&self, order_id: &uuid::Uuid) -> TradingResult<&Order>;
    
    /// Get all active orders
    async fn get_orders(&self) -> TradingResult<Vec<&Order>>;
    
    /// Get trades for a specific order
    async fn get_order_trades(&self, order_id: &uuid::Uuid) -> TradingResult<Vec<&Trade>>;
    
    /// Get all trades
    async fn get_trades(&self) -> TradingResult<Vec<&Trade>>;
    
    /// Get order book depth
    async fn get_order_book(&self, depth: usize) -> TradingResult<Vec<(Numeric, Numeric)>>;
}

pub use order_book::OrderBook;
pub use matching_engine::MatchingEngine;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Numeric, Order, OrderSide, OrderType};
    use uuid::Uuid;

    #[test]
    fn test_basic_matching() {
        let mut engine = MatchingEngine::new();
        let symbol = "BTCUSD".to_string();
        
        let buy_order = Order::new(
            &symbol,
            Numeric::from_str("50000").unwrap(),
            Numeric::from_str("1").unwrap(),
            OrderSide::Buy,
            OrderType::Limit,
            None,
        );


        let sell_order = Order::new(
            &symbol,
            Numeric::from_str("50000").unwrap(),
            Numeric::from_str("1").unwrap(),
            OrderSide::Sell,
            OrderType::Limit,
            None,
        );

        let trades = engine.add_order(symbol.clone(), buy_order);
        assert_eq!(trades.len(), 0);

        let trades = engine.add_order(symbol.clone(), sell_order);
        assert_eq!(trades.len(), 1);
    }
}
