use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub uuid: Uuid,                         // 唯一标识符
    pub base: String,                       // 基础字段，根据实际情况调整类型
    pub symbol: String,                     // 交易对符号
    pub order_id: String,                   // 订单ID
    pub order_side: String,                 // 订单方向
    pub order_type: String,                 // 订单类型（价格策略，市价单，限价单）
    pub user_id: String,                    // 用户ID
    pub price: String,                      // 价格
    pub quantity: String,                   // 数量
    pub fee_rate: String,                   // 手续费率
    pub amount: String,                     // 金额
    pub freeze_qty: String,                 // 冻结数量
    pub freeze_amount: String,              // 冻结金额
    pub avg_price: String,                  // 平均价格
    pub finished_qty: String,               // 完成数量
    pub finished_amount: String,            // 完成金额
    pub fee: String,                        // 手续费
    pub status: i16,                        // 状态，0-未成交，1-已成交，2-已取消，3-已撤销，4-已过期
    pub nano_time: i64,                     // 纳秒时间戳
}
