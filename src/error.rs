use thiserror::Error;

#[derive(Error, Debug)]
pub enum TradingError {
    #[error("Invalid order: {0}")]
    InvalidOrder(String),
    
    #[error("Order not found: {0}")]
    OrderNotFound(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Order already exists: {0}")]
    OrderExists(String),
    
    #[error("Order already completed")]
    OrderCompleted,
    
    #[error("Order already canceled")]
    OrderCanceled,
    
    #[error("Invalid price: {0}")]
    InvalidPrice(String),
    
    #[error("Invalid quantity: {0}")]
    InvalidQuantity(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type TradingResult<T> = std::result::Result<T, TradingError>;
