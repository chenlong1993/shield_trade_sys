use shield_trade_sys::service::order;

#[test]
fn test_get_user_id_key_integration() -> () {
    // 测试正常情况，长度大于等于3的用户ID
    let user_id_1 = "12345";
    match order::get_user_id_key(user_id_1) {
        Ok(result) => {
            println!("正常情况，结果为: {}", result);
            assert_eq!(result, "345");
        }
        Err(err) => {
            panic!("不应该出现错误，实际得到错误: {}", err);
        }
    }

    // 测试长度小于3的用户ID
    let user_id_2 = "12";
    match order::get_user_id_key(user_id_2) {
        Ok(result) => {
            println!("正常情况，结果为: {}", result);
            assert_eq!(result, "012");
        }
        Err(err) => {
            panic!("不应该出现错误，实际得到错误: {}", err);
        }
    }

    // 测试空的用户ID，验证错误返回情况
    let user_id_3 = "";
    match order::get_user_id_key(user_id_3) {
        Ok(_) => {
            panic!("应该返回错误，实际却成功了");
        }
        Err(err) => {
            assert_eq!(err, "user_id不能为空");
        }
    }
}
#[test]
fn test_get_order_id() -> () {
    match order::get_order_id() {
        Ok(order_id) => {
            println!("order_id: {}", order_id);
            assert!(order_id.len() >0)
        }
        Err(e) => {
            println!("Failed to get order_id: {}", e);
            assert!(false, "Expected Ok, but got Err: {}", e);
        }
    }
}
