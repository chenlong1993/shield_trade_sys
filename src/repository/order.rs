use crate::types::{Numeric, OrderSide, OrderType};
use crate::error::AppError;
use super::Order;
use super::OrderRepository;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub struct PostgresOrderRepository {
    pool: PgPool,
}

impl PostgresOrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn create_limit_order(
        &self,
        user_id: &str,
        symbol: &str,
        side: OrderSide,
        price: Numeric,
        quantity: Numeric
    ) -> Result<Order, AppError> {
        let order = sqlx::query!(
            r#"
            INSERT INTO orders 
            (order_id, user_id, symbol, side, order_type, price, quantity, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_id,
            symbol,
            side as i16,
            OrderType::Limit as i16,
            price,
            quantity,
            "NEW",
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(Order {
            order_id: order.order_id,
            user_id: order.user_id,
            symbol: order.symbol,
            side: OrderSide::from(order.side),
            order_type: OrderType::from(order.order_type),
            price: Some(order.price),
            quantity: Some(order.quantity),
            amount: None,
            status: OrderStatus::from(order.status),
            created_at: order.created_at.timestamp(),
        })
    }

    async fn create_market_by_amount(
        &self,
        user_id: &str,
        symbol: &str,
        side: OrderSide,
        amount: Numeric
    ) -> Result<Order, AppError> {
        let order = sqlx::query!(
            r#"
            INSERT INTO orders 
            (order_id, user_id, symbol, side, order_type, amount, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_id,
            symbol,
            side as i16,
            OrderType::Market as i16,
            amount,
            "NEW",
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(Order {
            order_id: order.order_id,
            user_id: order.user_id,
            symbol: order.symbol,
            side: OrderSide::from(order.side),
            order_type: OrderType::from(order.order_type),
            price: None,
            quantity: None,
            amount: Some(order.amount),
            status: OrderStatus::from(order.status),
            created_at: order.created_at.timestamp(),
        })
    }

    async fn create_market_by_quantity(
        &self,
        user_id: &str,
        symbol: &str,
        side: OrderSide,
        quantity: Numeric
    ) -> Result<Order, AppError> {
        let order = sqlx::query!(
            r#"
            INSERT INTO orders 
            (order_id, user_id, symbol, side, order_type, quantity, status, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            Uuid::new_v4(),
            user_id,
            symbol,
            side as i16,
            OrderType::Market as i16,
            quantity,
            "NEW",
            Utc::now()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(Order {
            order_id: order.order_id,
            user_id: order.user_id,
            symbol: order.symbol,
            side: OrderSide::from(order.side),
            order_type: OrderType::from(order.order_type),
            price: None,
            quantity: Some(order.quantity),
            amount: None,
            status: OrderStatus::from(order.status),
            created_at: order.created_at.timestamp(),
        })
    }

    async fn history_list(
        &self,
        user_id: &str,
        symbol: &str,
        start: i64,
        end: i64,
        limit: usize
    ) -> Result<Vec<Order>, AppError> {
        let orders = sqlx::query!(
            r#"
            SELECT * FROM orders
            WHERE user_id = $1
            AND symbol = $2
            AND created_at BETWEEN to_timestamp($3) AND to_timestamp($4)
            ORDER BY created_at DESC
            LIMIT $5
            "#,
            user_id,
            symbol,
            start,
            end,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(orders.into_iter().map(|order| Order {
            order_id: order.order_id,
            user_id: order.user_id,
            symbol: order.symbol,
            side: OrderSide::from(order.side),
            order_type: OrderType::from(order.order_type),
            price: order.price,
            quantity: order.quantity,
            amount: order.amount,
            status: OrderStatus::from(order.status),
            created_at: order.created_at.timestamp(),
        }).collect())
    }

    async fn unfinished_list(
        &self,
        symbol: &str
    ) -> Result<Vec<Order>, AppError> {
        let orders = sqlx::query!(
            r#"
            SELECT * FROM orders
            WHERE symbol = $1
            AND status IN ('NEW', 'PARTIALLY_FILLED')
            ORDER BY created_at ASC
            "#,
            symbol
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(orders.into_iter().map(|order| Order {
            order_id: order.order_id,
            user_id: order.user_id,
            symbol: order.symbol,
            side: OrderSide::from(order.side),
            order_type: OrderType::from(order.order_type),
            price: order.price,
            quantity: order.quantity,
            amount: order.amount,
            status: OrderStatus::from(order.status),
            created_at: order.created_at.timestamp(),
        }).collect())
    }
}
