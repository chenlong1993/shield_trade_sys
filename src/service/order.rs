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
    while userIdKey.len()!= 3 {
        userIdKey.insert(0, '0');
    }
    Ok(userIdKey)
}