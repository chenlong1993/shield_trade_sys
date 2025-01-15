// 单元测试模块
use shield_trade_sys::common::order_status::OrderStatus;
#[test]
fn test_order_status_as_str() {
    assert_eq!(OrderStatus::Unfilled.as_str(), "未成交");
    assert_eq!(OrderStatus::Filled.as_str(), "已成交");
    assert_eq!(OrderStatus::Canceled.as_str(), "已取消");
    assert_eq!(OrderStatus::Revoked.as_str(), "已撤销");
    assert_eq!(OrderStatus::Expired.as_str(), "已过期");
}

#[test]
fn test_order_status_values() {
    assert_eq!(OrderStatus::Unfilled as i16, 0);
    assert_eq!(OrderStatus::Filled as i16, 1);
    assert_eq!(OrderStatus::Canceled as i16, 2);
    assert_eq!(OrderStatus::Revoked as i16, 3);
    assert_eq!(OrderStatus::Expired as i16, 4);
}
