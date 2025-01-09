use serde::{Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
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
//
// impl Serialize for TradingError {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             TradingError::InvalidOrder(msg) => serializer.serialize_str(msg),
//             TradingError::OrderNotFound(msg) => serializer.serialize_str(msg),
//             TradingError::InsufficientFunds => serializer.serialize_str("Insufficient funds"),
//             TradingError::OrderExists(msg) => serializer.serialize_str(msg),
//             TradingError::OrderCompleted => serializer.serialize_str("Order already completed"),
//             TradingError::OrderCanceled => serializer.serialize_str("Order already canceled"),
//             TradingError::InvalidPrice(msg) => serializer.serialize_str(msg),
//             TradingError::InvalidQuantity(msg) => serializer.serialize_str(msg),
//             TradingError::DatabaseError(msg) => serializer.serialize_str(msg),
//             TradingError::InternalError(msg) => serializer.serialize_str(msg),
//         }
//     }
// }

pub type TradingResult<T> = std::result::Result<T, TradingError>;
