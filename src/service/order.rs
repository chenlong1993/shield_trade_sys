use chrono::{DateTime, Local, TimeZone, Utc};
use idgen_rs::id_helper;
use idgen_rs::options::IGOptions;

pub fn get_user_id_key(user_id: &str) -> Result<String, &'static str> {
    if user_id.is_empty() {
        return Err("user_id不能为空");
    }
    // 如果user_id的长度大于等于3，则直接返回后3位字符
    if user_id.len() >= 3 {
        let mut chars = user_id.chars().rev();
        let taken_chars: Vec<char> = chars.take(3).collect();
        return Ok(taken_chars.into_iter().rev().collect::<String>());
    }

    // 如果user_id的长度小于3，前面补0直到长度为3
    let mut userIdKey = user_id.to_string();
    while userIdKey.len() != 3 {
        userIdKey.insert(0, '0');
    }
    Ok(userIdKey)
}
//用fastsend生成唯一订单号，考虑多线程环境
pub fn get_order_id() -> Result<String, &'static str> {
    let mut options = IGOptions::new(1); // 1 是 worker id
    options.worker_id_bit_length = 10; // 默认值6，限定 WorkerId 最大值为2^6-1，即默认最多支持64个节点。
    options.seq_bit_length = 6; // 默认值6，限制每毫秒生成的ID个数。若生成速度超过5万个/秒，建议加大 SeqBitLength 到 10。
    let base_time: DateTime<Utc> = Utc
        .with_ymd_and_hms(2023, 3, 13, 3, 3, 3)
        .single()
        .expect("Failed to create DateTime<Utc>");
    options.base_time = base_time.timestamp_millis();

    // 保存参数（务必调用，否则参数设置不生效）：
    id_helper::set_options(options);
    // 初始化后，在任何需要生成ID的地方，调用以下方法：
    let new_id = id_helper::next_id();
    println!("new_id: {}", new_id);
    //返回订单号
    Ok(new_id.to_string())
}
