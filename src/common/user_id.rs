use actix_web::{HttpRequest, HttpResponse, Responder};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

// 定义 JWT 的 Claims 结构体
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    userId: String,
    exp: usize, // 过期时间
}

// 从 HTTP 请求中提取用户 ID
async fn get_user_id(req: HttpRequest) -> impl Responder {
    // 从请求头中获取 JWT 令牌
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .unwrap_or("");

    // 假设令牌是以 "Bearer " 开头
    let token = auth_header.trim_start_matches("Bearer ");

    // 你的秘密钥匙
    let secret_key = "your_secret_key";

    // 解码令牌
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    // 处理解码结果
    match token_data {
        Ok(data) => HttpResponse::Ok().body(format!("User ID: {}", data.claims.userId)),
        Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
    }
}