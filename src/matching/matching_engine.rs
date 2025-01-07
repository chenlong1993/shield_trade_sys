use crate::error::TradingError;
use crate::matching::order_book::OrderBook;
use crate::types::{Order, Trade};
use std::collections::HashMap;
use uuid::Uuid;

pub struct MatchingEngine {
    order_books: HashMap<String, OrderBook>,
}

#[async_trait::async_trait]
impl TradingEngine for MatchingEngine {
    async fn add_order(&mut self, order: Order) -> Result<Vec<Trade>, TradingError> {
        let order_book = self.order_books
            .entry(order.symbol.clone())
            .or_insert_with(OrderBook::new);
        
        order_book.add_order(order).await
    }

    async fn cancel_order(&mut self, symbol: &str, order_id: Uuid) -> Result<(), TradingError> {
        if let Some(order_book) = self.order_books.get_mut(symbol) {
            order_book.cancel_order(order_id).await
        } else {
            Err(TradingError::OrderNotFound(order_id.to_string()))
        }
    }

    async fn get_order(&self, order_id: Uuid) -> Result<Order, TradingError> {
        for order_book in self.order_books.values() {
            if let Some(order) = order_book.get_order(order_id).await {
                return Ok(order);
            }
        }
        Err(TradingError::OrderNotFound(order_id.to_string()))
    }

    async fn get_orders(&self) -> Vec<Order> {
        self.order_books.values()
            .flat_map(|ob| ob.get_orders().await)
            .collect()
    }

    async fn get_trades(&self) -> Vec<Trade> {
        self.order_books.values()
            .flat_map(|ob| ob.get_trades().await)
            .collect()
    }

    async fn get_orderbook(&self, symbol: &str) -> Result<OrderBookSnapshot, TradingError> {
        self.order_books.get(symbol)
            .map(|ob| ob.snapshot())
            .ok_or_else(|| TradingError::SymbolNotFound(symbol.to_string()))
    }
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            order_books: HashMap::new(),
        }
    }
}

use super::order_book::OrderBookSnapshot;
