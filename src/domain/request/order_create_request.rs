use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCreateRequest {
    #[serde(rename = "user_id")]
    pub user_id: String,  // 交易对符号，例如 "btcusdt"
    #[serde(rename = "symbol")]
    pub symbol: String,  // 交易对符号，例如 "btcusdt"

    #[serde(rename = "side")]
    pub side: String,  // 订单方向，例如 "buy"

    #[serde(rename = "order_type")]
    pub order_type: String,  // 订单类型，例如 "limit"

    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,  // 价格，可能为空

    #[serde(rename = "qty", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,  // 数量，可能为空

    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,  // 金额，可能为空
}