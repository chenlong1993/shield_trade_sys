//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "kline")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: String,
    pub base: String,
    pub symbol: String,
    pub period: String,
    pub open_at: i64,
    pub close_at: i64,
    #[sea_orm(column_type = "Double")]
    pub open: f64,
    #[sea_orm(column_type = "Double")]
    pub high: f64,
    #[sea_orm(column_type = "Double")]
    pub low: f64,
    #[sea_orm(column_type = "Double")]
    pub close: f64,
    #[sea_orm(column_type = "Double")]
    pub volume: f64,
    #[sea_orm(column_type = "Double")]
    pub amount: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
