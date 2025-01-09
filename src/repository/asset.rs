use crate::repository::base::{Base, UUID};
use crate::types::Numeric;

#[derive(Debug)]
pub struct Asset {
    uuid: UUID,      // 对应原来的models.UUID，这里用Uuid类型表示，假设外部会正确处理生成等逻辑
    base: Base,      // 对应原来的models.Base，使用之前定义的Base结构体（假设已经正确定义并可使用）
    user_id: String, // 用户id
    symbol: String,  // 货币符号
    total_balance: Numeric, // 总余额
    freeze_balance: Numeric, // 冻结余额
    avail_balance: Numeric, // 可用余额
}
