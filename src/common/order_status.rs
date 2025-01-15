#[derive(Debug)]
#[repr(i16)]
pub enum OrderStatus {
    Unfilled = 0, // 未成交
    Filled = 1,   // 已成交
    Canceled = 2, // 已取消
    Revoked = 3,  // 已撤销
    Expired = 4,  // 已过期
}

impl OrderStatus {
    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Unfilled => "未成交",
            OrderStatus::Filled => "已成交",
            OrderStatus::Canceled => "已取消",
            OrderStatus::Revoked => "已撤销",
            OrderStatus::Expired => "已过期",
        }
    }
}
