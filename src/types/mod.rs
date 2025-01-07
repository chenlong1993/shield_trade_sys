use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Numeric(Decimal);

impl Numeric {
    pub fn from_str(s: &str) -> Result<Self, rust_decimal::Error> {
        Decimal::from_str(s).map(Numeric)
    }

    pub fn zero() -> Self {
        Numeric(Decimal::ZERO)
    }

    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl FromStr for Numeric {
    type Err = rust_decimal::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Decimal::from_str(s).map(Numeric)
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Numeric {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Numeric(self.0 + rhs.0)
    }
}

impl Sub for Numeric {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Numeric(self.0 - rhs.0)
    }
}

impl Mul for Numeric {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Numeric(self.0 * rhs.0)
    }
}

impl Div for Numeric {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Numeric(self.0 / rhs.0)
    }
}

impl Rem for Numeric {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Numeric(self.0 % rhs.0)
    }
}

impl Neg for Numeric {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Numeric(-self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: Uuid,
    pub symbol: String,
    pub price: Numeric,
    pub quantity: Numeric,
    pub amount: Option<Numeric>,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub timestamp: u64,
    pub created_at: u64,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trade {
    pub id: Uuid,
    pub symbol: String,
    pub price: Numeric,
    pub quantity: Numeric,
    pub taker_order_id: Uuid,
    pub maker_order_id: Uuid,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
}

impl Order {
    pub fn new(
        symbol: &str,
        price: Numeric,
        quantity: Numeric,
        side: OrderSide,
        order_type: OrderType,
        amount: Option<Numeric>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            symbol: symbol.to_string(),
            price,
            quantity,
            amount,
            side,
            order_type,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status: OrderStatus::New,
        }
    }

    pub fn new_market_by_amount(
        symbol: &str,
        amount: Numeric,
        side: OrderSide,
    ) -> Self {
        Self::new(
            symbol,
            Numeric::zero(),
            Numeric::zero(),
            side,
            OrderType::Market,
            Some(amount),
        )
    }

    pub fn new_market_by_qty(
        symbol: &str,
        quantity: Numeric,
        side: OrderSide,
    ) -> Self {
        Self::new(
            symbol,
            Numeric::zero(),
            quantity,
            side,
            OrderType::Market,
            None,
        )
    }
}
