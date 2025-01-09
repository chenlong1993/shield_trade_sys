use std::str::FromStr;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use serde_json::json;
use crate::types::Numeric;

pub struct UserAssetsController {
    repo: Arc<dyn AssetRepository>,
    logger: Arc<env_logger::Logger>,
}

//定义相关的接口
pub trait AssetRepository: Send + Sync {
    fn deposit(&self, transaction_id: &str, user_id: &str, symbol: &str, amount: Numeric) -> Result<(), String>;
    fn withdraw(&self, transaction_id: &str, user_id: &str, symbol: &str, amount: Numeric) -> Result<(), String>;
    fn transfer(&self, transaction_id: &str, from: &str, to: &str, symbol: &str, amount: Numeric) -> Result<(), String>;
    fn query_assets(&self, user_id: &str, symbols: Vec<&str>) -> Result<Vec<Asset>, String>;
}

#[derive(Debug, Serialize)]
pub struct Asset {
    user_id: String,
    symbol: String,
    balance: Numeric,
    // Add other fields as needed
}

#[derive(Debug, Deserialize)]
pub struct DepositRequest {
    user_id: String,
    symbol: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
pub struct WithdrawRequest {
    user_id: String,
    symbol: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    from: String,
    to: String,
    symbol: String,
    amount: String,
}

impl UserAssetsController {
    pub fn new(repo: Arc<dyn AssetRepository>, logger: Arc<env_logger::Logger>) -> Self {
        Self { repo, logger }
    }

    //存款
    pub async fn deposit(
        data: web::Data<Arc<Self>>,
        req: web::Json<DepositRequest>
    ) -> impl Responder {
        let transaction_id = Uuid::new_v4().to_string();
        match data.repo.deposit(
            &transaction_id,
            &req.user_id,
            &req.symbol,
            Numeric::from_str(&req.amount).unwrap_or_default()
        ) {
            Ok(_) => HttpResponse::Ok().json(transaction_id),
            Err(e) => HttpResponse::BadRequest().json(json!({"error": e}))
        }
    }

    //提取
    pub async fn withdraw(
        data: web::Data<Arc<Self>>,
        req: web::Json<WithdrawRequest>
    ) -> impl Responder {
        let transaction_id = Uuid::new_v4().to_string();
        match data.repo.withdraw(
            &transaction_id,
            &req.user_id,
            &req.symbol,
            Numeric::from_str(&req.amount).unwrap_or_default()
        ) {
            Ok(_) => HttpResponse::Ok().json(transaction_id),
            Err(e) => HttpResponse::BadRequest().json(json!({"error": e}))
        }
    }

    //交易
    pub async fn transfer(
        data: web::Data<Arc<Self>>,
        req: web::Json<TransferRequest>
    ) -> impl Responder {
        let transaction_id = Uuid::new_v4().to_string();
        match data.repo.transfer(
            &transaction_id,
            &req.from,
            &req.to,
            &req.symbol,
            Numeric::from_str(&req.amount).unwrap_or_default()
        ) {
            Ok(_) => HttpResponse::Ok().json(transaction_id),
            Err(e) => HttpResponse::BadRequest().json(json!({"error": e}))
        }
    }

    //查询历史
    pub async fn query_assets(
        data: web::Data<Arc<Self>>,
        user_id: web::Path<String>,
        symbols: web::Query<String>
    ) -> impl Responder {
        let symbols: Vec<&str> = symbols.split(',').collect();
        match data.repo.query_assets(&user_id, symbols) {
            Ok(assets) => HttpResponse::Ok().json(assets),
            Err(e) => HttpResponse::BadRequest().json(json!({"error": e}))
        }
    }
}
