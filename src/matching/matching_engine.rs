use crate::error::TradingResult;
use crate::matching::order_book::OrderBook;
use crate::matching::TradingEngine;
use crate::types::{Order, Trade};
use std::collections::HashMap;
use uuid::Uuid;

pub struct MatchingEngine {
    order_books: HashMap<String, OrderBook>,
}

#[async_trait::async_trait]
impl TradingEngine for MatchingEngine {
    async fn get_order_trades(&self, _order_id: &Uuid) -> TradingResult<Vec<&Trade>> {
        todo!()
    }

    async fn get_order(&self, _order_id: &Uuid) -> TradingResult<&Order> {
        todo!()
    }

    async fn add_order(&mut self, _order: Order) -> TradingResult<()> {
        todo!()
    }

    async fn get_trades(&self) -> TradingResult<Vec<&Trade>> {
        todo!()
    }

    async fn get_orders(&self) -> TradingResult<Vec<&Order>> {
        todo!()
    }
}

impl MatchingEngine {
    pub fn new() -> Self {
        Self {
            order_books: HashMap::new(),
        }
    }
}
