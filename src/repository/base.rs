use chrono::prelude::*;
use serde::{Deserialize, Serialize};

// 对应Base结构体，定义一个公共的基础结构体，包含创建时间和更新时间
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Base {
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}
//对应公共UUID
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UUID {
    pub id: String,
}