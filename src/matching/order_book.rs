use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;
use crate::types::{Order, OrderSide, OrderStatus};
use crate::error::TradingError;
use crate::types::Trade;
use crate::types::Numeric;
use std::cmp::Ordering;

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

    pub async fn cancel_order(&mut self, order_id: Uuid) -> Result<(), TradingError> {
        if let Some(order) = self.order_map.get_mut(&order_id) {
            order.status = OrderStatus::Canceled;
            
            match order.side {
                OrderSide::Buy => self.bids.get_mut(&order.price).map(|orders| {
                    orders.retain(|o| o.id != order_id);
                }),
                OrderSide::Sell => self.asks.get_mut(&order.price).map(|orders| {
                    orders.retain(|o| o.id != order_id);
                }),
            };
            
            self.order_map.remove(&order_id);
            Ok(())
        } else {
            Err(TradingError::OrderNotFound(order_id.to_string()))
        }
    }

    pub fn snapshot(&self) -> OrderBookSnapshot {
        let bids = self.bids
            .iter()
            .rev()
            .take(10)
            .map(|(price, orders)| {
                let total_qty: Numeric = orders.iter()
                    .map(|o| &o.quantity)
                    .sum();
                (price.to_string(), total_qty.to_string())
            })
            .collect();

        let asks = self.asks
            .iter()
            .take(10)
            .map(|(price, orders)| {
                let total_qty: Numeric = orders.iter()
                    .map(|o| &o.quantity)
                    .sum();
                (price.to_string(), total_qty.to_string())
            })
            .collect();

        OrderBookSnapshot {
            bids,
            asks,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    async fn match_order(&mut self, order: Order) -> Result<Vec<Trade>, TradingError> {
        let mut trades = Vec::new();
        let mut remaining_qty = order.quantity.clone();
        
        let (opposite_orders, same_side_orders) = match order.side {
            OrderSide::Buy => (&mut self.asks, &mut self.bids),
            OrderSide::Sell => (&mut self.bids, &mut self.asks),
        };

        while remaining_qty > Numeric::from(0) {
            let (best_price, orders) = match order.side {
                OrderSide::Buy => opposite_orders.first_entry(),
                OrderSide::Sell => opposite_orders.last_entry(),
            };

            if let Some((best_price, orders)) = best_price {
                // Check price condition
                let price_condition = match order.side {
                    OrderSide::Buy => *best_price <= order.price,
                    OrderSide::Sell => *best_price >= order.price,
                };
                
                if !price_condition {
                    break;
                }

                let mut matched = false;
                for opposite_order in orders.iter_mut() {
                    if opposite_order.quantity <= remaining_qty {
                        // Full match
                        trades.push(self.create_trade(&order, opposite_order, &opposite_order.quantity));
                        remaining_qty -= &opposite_order.quantity;
                        opposite_order.quantity = Numeric::from(0);
                        opposite_order.status = OrderStatus::Filled;
                        matched = true;
                    } else {
                        // Partial match
                        trades.push(self.create_trade(&order, opposite_order, &remaining_qty));
                        opposite_order.quantity -= &remaining_qty;
                        remaining_qty = Numeric::from(0);
                        matched = true;
                        break;
                    }
                }

                if matched {
                    orders.retain(|o| o.quantity > Numeric::from(0));
                    if orders.is_empty() {
                        opposite_orders.remove(best_price);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if remaining_qty > Numeric::from(0) {
            same_side_orders
                .entry(order.price.clone())
                .or_insert_with(Vec::new)
                .push(Order {
                    quantity: remaining_qty,
                    ..order
                });
        }

        Ok(trades)
    }

    fn create_trade(&self, taker: &Order, maker: &Order, qty: &Numeric) -> Trade {
        Trade {
            id: Uuid::new_v4(),
            taker_order_id: taker.id,
            maker_order_id: maker.id,
            price: maker.price.clone(),
            quantity: qty.clone(),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}
