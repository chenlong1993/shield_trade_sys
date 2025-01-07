use crate::error::TradingError;
use crate::matching::order_book::OrderBook;
use crate::types::{Order, Trade};
use std::collections::HashMap;
use uuid::Uuid;

pub struct MatchingEngine {
    order_books: HashMap<String, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            order_books: HashMap::new(),
        }
    }

    pub async fn add_order(&mut self, order: Order) -> Result<Vec<Trade>, TradingError> {
        let order_book = self.order_books
            .entry(order.symbol.clone())
            .or_insert_with(OrderBook::new);
        
        order_book.add_order(order).await
    }

    pub async fn cancel_order(&mut self, symbol: &str, order_id: Uuid) -> Result<(), TradingError> {
        if let Some(order_book) = self.order_books.get_mut(symbol) {
            order_book.cancel_order(order_id).await
        } else {
            Err(TradingError::OrderNotFound(order_id.to_string()))
        }
    }

    pub async fn get_order_book_snapshot(&self, symbol: &str) -> Option<OrderBookSnapshot> {
        self.order_books.get(symbol).map(|ob| ob.snapshot())
    }
}

use super::order_book::OrderBookSnapshot;
