use std::collections::HashMap;
use crossbeam_channel::{Sender, Receiver};
use crate::types::{Order, Trade, OrderStatus};
use crate::error::{TradingError, TradingResult};
use super::{TradingEngine, OrderBook};
use uuid::Uuid;

pub struct MatchingEngine {
    order_books: HashMap<String, OrderBook>,
    trades: Vec<Trade>,
    order_map: HashMap<Uuid, Order>,
    command_tx: Sender<EngineCommand>,
    result_rx: Receiver<EngineResult>,
}

enum EngineCommand {
    AddOrder(Order),
    CancelOrder(Uuid),
    GetOrder(Uuid),
    GetOrders,
    GetOrderTrades(Uuid),
    GetTrades,
    GetOrderBook(usize),
}

enum EngineResult {
    Ok,
    Order(Order),
    Orders(Vec<Order>),
    Trades(Vec<Trade>),
    OrderBook(Vec<(Numeric, Numeric)>),
    Error(TradingError),
}

impl MatchingEngine {
    pub fn new() -> Self {
        let (command_tx, command_rx) = crossbeam_channel::unbounded();
        let (result_tx, result_rx) = crossbeam_channel::unbounded();
        
        // Start engine processing thread
        std::thread::spawn(move || {
            let mut engine = MatchingEngine {
                order_books: HashMap::new(),
                trades: Vec::new(),
                order_map: HashMap::new(),
                command_tx: command_tx.clone(),
                result_rx: result_rx.clone(),
            };
            
            while let Ok(cmd) = command_rx.recv() {
                let result = match cmd {
                    EngineCommand::AddOrder(order) => engine.handle_add_order(order),
                    EngineCommand::CancelOrder(order_id) => engine.handle_cancel_order(&order_id),
                    EngineCommand::GetOrder(order_id) => engine.handle_get_order(&order_id),
                    EngineCommand::GetOrders => engine.handle_get_orders(),
                    EngineCommand::GetOrderTrades(order_id) => engine.handle_get_order_trades(&order_id),
                    EngineCommand::GetTrades => engine.handle_get_trades(),
                    EngineCommand::GetOrderBook(depth) => engine.handle_get_order_book(depth),
                };
                
                result_tx.send(result).unwrap();
            }
        });

        Self {
            order_books: HashMap::new(),
            trades: Vec::new(),
            order_map: HashMap::new(),
            command_tx,
            result_rx,
        }
    }

    fn handle_add_order(&mut self, order: Order) -> EngineResult {
        let order_book = self.order_books
            .entry(order.symbol.clone())
            .or_insert_with(OrderBook::new);
            
        let trades = order_book.add_order(order.clone());
        self.trades.extend(trades.clone());
        self.order_map.insert(order.id, order);
        
        EngineResult::Trades(trades)
    }

    fn handle_cancel_order(&mut self, order_id: &Uuid) -> EngineResult {
        if let Some(order) = self.order_map.get_mut(order_id) {
            if order.status == OrderStatus::Canceled {
                return EngineResult::Error(TradingError::OrderCanceled);
            }
            
            if let Some(order_book) = self.order_books.get_mut(&order.symbol) {
                order_book.cancel_order(order_id);
                order.status = OrderStatus::Canceled;
                return EngineResult::Order(order.clone());
            }
        }
        EngineResult::Error(TradingError::OrderNotFound(order_id.to_string()))
    }

    fn handle_get_order(&self, order_id: &Uuid) -> EngineResult {
        self.order_map
            .get(order_id)
            .map(|o| EngineResult::Order(o.clone()))
            .unwrap_or_else(|| {
                EngineResult::Error(TradingError::OrderNotFound(order_id.to_string()))
            })
    }

    fn handle_get_orders(&self) -> EngineResult {
        EngineResult::Orders(self.order_map.values().cloned().collect())
    }

    fn handle_get_order_trades(&self, order_id: &Uuid) -> EngineResult {
        let trades = self.trades
            .iter()
            .filter(|t| t.taker_order_id == *order_id || t.maker_order_id == *order_id)
            .cloned()
            .collect();
            
        EngineResult::Trades(trades)
    }

    fn handle_get_trades(&self) -> EngineResult {
        EngineResult::Trades(self.trades.clone())
    }

    fn handle_get_order_book(&self, depth: usize) -> EngineResult {
        // TODO: Implement order book depth view
        EngineResult::OrderBook(Vec::new())
    }
}

#[async_trait::async_trait]
impl TradingEngine for MatchingEngine {
    async fn add_order(&mut self, order: Order) -> TradingResult<()> {
        self.command_tx.send(EngineCommand::AddOrder(order)).unwrap();
        match self.result_rx.recv().unwrap() {
            EngineResult::Ok => Ok(()),
            EngineResult::Error(e) => Err(e),
            _ => Err(TradingError::InternalError("Unexpected result".to_string())),
        }
    }

    async fn cancel_order(&mut self, order_id: &Uuid) -> TradingResult<Order> {
        self.command_tx.send(EngineCommand::CancelOrder(*order_id)).unwrap();
        match self.result_rx.recv().unwrap() {
            EngineResult::Order(order) => Ok(order),
            EngineResult::Error(e) => Err(e),
            _ => Err(TradingError::InternalError("Unexpected result".to_string())),
        }
    }

    async fn get_order(&self, order_id: &Uuid) -> TradingResult<&Order> {
        self.command_tx.send(EngineCommand::GetOrder(*order_id)).unwrap();
        match self.result_rx.recv().unwrap() {
            EngineResult::Order(order) => Ok(&order),
            EngineResult::Error(e) => Err(e),
            _ => Err(TradingError::InternalError("Unexpected result".to_string())),
        }
    }

    async fn get_orders(&self) -> TradingResult<Vec<&Order>> {
        self.command_tx.send(EngineCommand::GetOrders).unwrap();
        match self.result_rx.recv().unwrap() {
            EngineResult::Orders(orders) => Ok(orders.iter().collect()),
            EngineResult::Error(e) => Err(e),
            _ => Err(TradingError::InternalError("Unexpected result".to_string())),
        }
    }

    async fn get_order_trades(&self, order_id: &Uuid) -> TradingResult<Vec<&Trade>> {
        self.command_tx.send(EngineCommand::GetOrderTrades(*order_id)).unwrap();
        match self.result_rx.recv().unwrap() {
            EngineResult::Trades(trades) => Ok(trades.iter().collect()),
            EngineResult::Error(e) => Err(e),
            _ => Err(TradingError::InternalError("Unexpected result".to_string())),
        }
    }

    async fn get_trades(&self) -> TradingResult<Vec<&Trade>> {
        self.command_tx.send(EngineCommand::GetTrades).unwrap();
        match self.result_rx.recv().unwrap() {
            EngineResult::Trades(trades) => Ok(trades.iter().collect()),
            EngineResult::Error(e) => Err(e),
            _ => Err(TradingError::InternalError("Unexpected result".to_string())),
        }
    }

    async fn get_order_book(&self, depth: usize) -> TradingResult<Vec<(Numeric, Numeric)>> {
        self.command_tx.send(EngineCommand::GetOrderBook(depth)).unwrap();
        match self.result_rx.recv().unwrap() {
            EngineResult::OrderBook(depth) => Ok(depth),
            EngineResult::Error(e) => Err(e),
            _ => Err(TradingError::InternalError("Unexpected result".to_string())),
        }
    }
}
