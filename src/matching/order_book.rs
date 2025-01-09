use crate::error::TradingError;
use crate::types::Numeric;
use crate::types::Order;
use crate::types::Trade;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

pub struct OrderBook {
    bids: BTreeMap<Numeric, Vec<Order>>,
    asks: BTreeMap<Numeric, Vec<Order>>,
    order_map: HashMap<Uuid, Order>,
}

#[derive(Debug, Clone)]
pub struct OrderBookSnapshot {
    pub bids: Vec<(String, String)>,
    pub asks: Vec<(String, String)>,
    pub timestamp: u64,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            order_map: HashMap::new(),
        }
    }

    pub async fn add_order(&mut self, order: Order) -> Result<Vec<Trade>, TradingError> {
        self.order_map.insert(order.id, order.clone());
        self.match_order(order).await
    }

    pub async fn cancel_order(&mut self, _order_id: Uuid) -> Result<(), TradingError> {
        todo!()
    }

    pub fn snapshot(&self) -> OrderBookSnapshot {
        //
        todo!()
    }

    async fn match_order(&mut self, _order: Order) -> Result<Vec<Trade>, TradingError> {
        todo!()
    }

    fn create_trade(&self, _taker: &Order, _maker: &Order, _qty: &Numeric) -> Trade {
        todo!()
    }
}
