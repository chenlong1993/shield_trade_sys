use std::collections::{BTreeMap, HashMap};
use crate::types::{Order, Trade, OrderSide, OrderStatus};
use crate::error::TradingResult;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug)]
pub struct OrderBook {
    bids: BTreeMap<Decimal, Vec<Order>>,
    asks: BTreeMap<Decimal, Vec<Order>>,
    orders: HashMap<Uuid, Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, mut order: Order) -> Vec<Trade> {
        self.orders.insert(order.id, order.clone());
        
        match order.side {
            OrderSide::Buy => self.match_buy_order(&mut order),
            OrderSide::Sell => self.match_sell_order(&mut order),
        }
    }

    pub fn cancel_order(&mut self, order_id: &Uuid) -> TradingResult<()> {
        if let Some(order) = self.orders.get_mut(order_id) {
            order.status = OrderStatus::Canceled;
            
            match order.side {
                OrderSide::Buy => self.remove_from_book(&mut self.bids, order),
                OrderSide::Sell => self.remove_from_book(&mut self.asks, order),
            }
            
            Ok(())
        } else {
            Err(TradingError::OrderNotFound(order_id.to_string()))
        }
    }

    fn match_buy_order(&mut self, order: &mut Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut remaining_qty = order.quantity;

        // Iterate through asks from lowest price
        for (price, orders) in self.asks.iter_mut() {
            if *price > order.price && order.order_type == OrderType::Limit {
                break;
            }

            while let Some(mut ask) = orders.pop() {
                let trade_qty = remaining_qty.min(ask.quantity);
                
                let trade = Trade {
                    id: Uuid::new_v4(),
                    symbol: order.symbol.clone(),
                    price: *price,
                    quantity: trade_qty,
                    taker_order_id: order.id,
                    maker_order_id: ask.id,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                trades.push(trade);
                
                // Update quantities
                remaining_qty = remaining_qty - trade_qty;
                ask.quantity = ask.quantity - trade_qty;
                
                // Update order statuses
                if ask.quantity.is_zero() {
                    ask.status = OrderStatus::Filled;
                } else {
                    orders.push(ask);
                }

                if remaining_qty.is_zero() {
                    order.status = OrderStatus::Filled;
                    break;
                }
            }

            if remaining_qty.is_zero() {
                break;
            }
        }

        // Add remaining quantity to order book
        if !remaining_qty.is_zero() && order.order_type == OrderType::Limit {
            order.quantity = remaining_qty;
            order.status = OrderStatus::PartiallyFilled;
            self.bids.entry(order.price)
                .or_insert_with(Vec::new)
                .push(order.clone());
        }

        trades
    }

    fn match_sell_order(&mut self, order: &mut Order) -> Vec<Trade> {
        let mut trades = Vec::new();
        let mut remaining_qty = order.quantity;

        // Iterate through bids from highest price
        for (price, orders) in self.bids.iter_mut().rev() {
            if *price < order.price && order.order_type == OrderType::Limit {
                break;
            }

            while let Some(mut bid) = orders.pop() {
                let trade_qty = remaining_qty.min(bid.quantity);
                
                let trade = Trade {
                    id: Uuid::new_v4(),
                    symbol: order.symbol.clone(),
                    price: *price,
                    quantity: trade_qty,
                    taker_order_id: order.id,
                    maker_order_id: bid.id,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                trades.push(trade);
                
                // Update quantities
                remaining_qty = remaining_qty - trade_qty;
                bid.quantity = bid.quantity - trade_qty;
                
                // Update order statuses
                if bid.quantity.is_zero() {
                    bid.status = OrderStatus::Filled;
                } else {
                    orders.push(bid);
                }

                if remaining_qty.is_zero() {
                    order.status = OrderStatus::Filled;
                    break;
                }
            }

            if remaining_qty.is_zero() {
                break;
            }
        }

        // Add remaining quantity to order book
        if !remaining_qty.is_zero() && order.order_type == OrderType::Limit {
            order.quantity = remaining_qty;
            order.status = OrderStatus::PartiallyFilled;
            self.asks.entry(order.price)
                .or_insert_with(Vec::new)
                .push(order.clone());
        }

        trades
    }

    fn remove_from_book(&mut self, book: &mut BTreeMap<Decimal, Vec<Order>>, order: &Order) {
        if let Some(orders) = book.get_mut(&order.price) {
            orders.retain(|o| o.id != order.id);
            if orders.is_empty() {
                book.remove(&order.price);
            }
        }
    }
}
