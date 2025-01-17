use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventOrderNew {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,

    #[serde(rename = "order_side", skip_serializing_if = "Option::is_none")]
    pub order_side: Option<String>,  // Adjust this type based on your OrderSide enum

    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,  // Adjust this type based on your OrderType enum

    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,

    #[serde(rename = "max_amount", skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<f64>,

    #[serde(rename = "max_qty", skip_serializing_if = "Option::is_none")]
    pub max_qty: Option<f64>,

    #[serde(rename = "nano_time", skip_serializing_if = "Option::is_none")]
    pub nano_time: Option<i64>,
}