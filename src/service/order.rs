use crate::domain::request::order_create_request::OrderCreateRequest;
use crate::models::event_order_new::EventOrderNew;
use crate::repository::base::UUID;
use crate::repository::{asset_freezes, order, trade_varieties};
use actix_web::{Error};
use chrono::{DateTime,  TimeZone, Utc};
use idgen_rs::id_helper;
use idgen_rs::options::IGOptions;
use log::error;
use rust_decimal::Decimal;
use sea_orm::sea_query::{ExprTrait, SimpleExpr};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait, ColumnTrait, DbErr};
use std::str::FromStr;
use rust_decimal::prelude::Zero;
use sea_orm::prelude::Expr;
use uuid::Uuid;


pub async fn create_order(
    db: &DatabaseConnection,
    req: OrderCreateRequest,
) -> Result<order::Model, sea_orm::DbErr> {
    let user_id = req.user_id.clone();
    //如果是限价单
    if req.order_type == "limit" {
        if req.price.is_none() || req.quantity.is_none() {
            return Err(DbErr::Custom("price or quantity isnull".parse().unwrap()));
        }

        // 创建限价订单逻辑
        let price = Decimal::from_str(&req.price.unwrap()).unwrap();
        let quantity = Decimal::from_str(&req.quantity.unwrap()).unwrap();

        // 假设 create_limit_order 返回订单和错误
        match create_limit_order(
            &db,
            &*user_id,
            &req.symbol,
            &req.side,
            f64::try_from(price.clone()).unwrap(),
            f64::try_from(quantity.clone()).unwrap(),
        )
            .await
        {
            Ok(order) => {
                let event = EventOrderNew {
                    symbol: Some(order.symbol.clone()),
                    order_id: Some(order.order_id.clone()),
                    order_side: Some(order.order_side.clone()),
                    order_type: Some(order.order_type.clone()),
                    nano_time: Some(order.nano_time.clone()),
                    price: Some(f64::try_from(price.clone()).unwrap()),
                    quantity: Some(f64::try_from(quantity.clone()).unwrap()),
                    amount: None,
                    max_amount: None,
                    max_qty: None,
                };

                publish_event(event).await;

                Ok(order)
            }
            Err(err) => {
                //打印错误日志
                error!("create limit order error: {:?}", err);
                Err(DbErr::Custom("create limit order error".parse().unwrap())).expect("TODO: panic message")
            }
        }
    } else {
        if req.amount.is_none() && req.quantity.is_none() {
            return  Err(DbErr::Custom("amount or quantity is required".parse().unwrap()))
        }

        // 创建市价订单逻辑
        if let Some(amount) = &req.amount {
            let amount = Decimal::from_str(&amount).unwrap();

            match create_market_order_by_amount(&*user_id, &req.symbol, &req.side, amount.clone())
                .await
            {
                Ok(order) => {
                    let event = EventOrderNew {
                        symbol: Some(order.symbol.clone()),
                        order_id: Some(order.order_id.clone()),
                        order_side: Some(order.order_id.clone()),
                        order_type: Some(order.order_type.clone()),
                        nano_time: Some(order.nano_time),
                        price: None,
                        quantity: None,
                        amount: Some(f64::try_from(amount.clone()).unwrap()),
                        max_amount: Some(order.freeze_amount.clone()),
                        max_qty: None,
                    };

                    publish_event(event).await;

                    Ok(order)
                }
                Err(err) => {
                    error!("create market order by amount error: {:?}", err);
                    Err(DbErr::Custom("create market order by amount error".parse().unwrap()))
                }
            }
        } else {
            let quantity = Decimal::from_str(&*req.quantity.unwrap()).unwrap();

            match create_market_order_by_qty(&*user_id, &req.symbol, &req.side, quantity.clone())
                .await
            {
                Ok(order) => {
                    let event = EventOrderNew {
                        symbol: Some(order.symbol.clone()),
                        order_id: Some(order.order_id.clone()),
                        order_side: Some(order.order_side.clone()),
                        order_type: Some(order.order_type.clone()),
                        nano_time: Some(order.nano_time.clone()),
                        price: None,
                        quantity: Some(f64::try_from(quantity.clone()).unwrap()),
                        amount: None,
                        max_amount: None,
                        max_qty: Some(order.freeze_qty),
                    };

                    publish_event(event).await;

                    Ok(order)
                }
                Err(err) => {
                    error!("create market order by qty error: {:?}", err);
                    Err(DbErr::Custom("create market order by qty error".parse().unwrap()))
                }
            }
        }
    }
}

// 假设的数据库操作和事件发布函数
async fn create_limit_order(
    db: &DatabaseConnection,
    user_id: &str,
    symbol: &str,
    side: &str,
    price: f64,
    quantity: f64,
) -> Result<order::Model, sea_orm::DbErr> {
    {
        // 实现创建限价订单的逻辑
        // 查询交易对配置
        let trade_info = find_by_symbol(&db, symbol).await;

        let order_id = get_order_id().unwrap();;
        let mut uid = UUID {
            id: String::from(Uuid::new_v4()),
        };

        //
        let fee_rate =trade_info?.get(0).unwrap().fee_rate;
        let mut data = order::Model {
            uuid: uid.id,
            order_id: order_id.clone(),
            user_id: user_id.to_string(),
            symbol: symbol.to_string(),
            order_side: side.to_string(),
            order_type: "limit".to_string(),
            price: price.clone(),
            quantity: quantity.clone(),
            nano_time: Utc::now().timestamp(),
            fee_rate: fee_rate.unwrap(),
            amount: Default::default(),
            freeze_qty: Default::default(),
            freeze_amount: Default::default(),
            avg_price: Default::default(),
            finished_qty: Default::default(),
            finished_amount: Default::default(),
            fee: Default::default(),
            status: 1,
            base: "".to_string(),
        };

        let unfinished = &data;

        validate_order_limit(&data, &db)
            .await
            .expect("TODO: panic message");

        //开始事物
        let mut tx = db.begin().await;

        // 冻结资产，如果是卖单直接冻结数量，如果是买单冻结数量+手续费
        if data.order_side == "sell" {
            data.freeze_qty = data.quantity.clone();
            //编写冻结资产的逻辑，也就是在冻结字段update
            freeze_amount(
                &data.order_id,
                &data.user_id,
                &data.symbol,
                f64::try_from(data.freeze_qty).unwrap(),
                &db,
            )
        } else {
            let amount = &data.price * &data.quantity;
            let fee = &amount * &data.fee_rate;
            data.freeze_amount = (amount + fee);
            //冻结资产
            freeze_amount(
                &data.order_id,
                &data.user_id,
                &data.symbol,
                data.freeze_qty,
                &db,
            )
        }
        .await
        .expect("TODO: panic message");

        let new_order = order::ActiveModel {
            uuid: Set(data.uuid.clone()),
            base: Set(data.base.clone()),
            symbol: Set(data.symbol.clone()),
            order_id: Set(data.order_id.clone()),
            order_side: Set(data.order_side.clone()),
            order_type: Set(data.order_type.clone()),
            user_id: Set(data.user_id.clone()),
            price: Set(data.price.to_string().parse().unwrap()),
            quantity: Set(data.quantity.to_string().parse().unwrap()),
            fee_rate: Set(data.fee_rate.to_string().parse().unwrap()),
            amount: Set(data.amount.to_string().parse().unwrap()),
            freeze_qty: Set(data.freeze_qty.to_string().parse().unwrap()),
            freeze_amount: Set(data.freeze_amount.to_string().parse().unwrap()),
            avg_price: Set(data.avg_price.to_string().parse().unwrap()),
            finished_qty: Set(data.finished_qty.to_string().parse().unwrap()),
            finished_amount: Set(data.finished_amount.to_string().parse().unwrap()),
            fee: Set(data.fee.to_string().parse().unwrap()),
            status: Set(data.status),
            nano_time: Set(data.nano_time),
        };

        new_order.insert(db).await?;

        // sqlx::query("INSERT INTO unfinished_orders (...) VALUES (...)")
        //     .bind(&unfinished.order_id)
        //     // Bind other fields
        //     .execute(&mut tx)
        //     .await?;

        tx.unwrap().commit().await?;
        Ok(data)
    }
}

pub async fn freeze_amount(
    trans_id: &str,
    user_id: &str,
    symbol: &str,
    amount: f64,
    db: &DatabaseConnection,
) -> Result<(), Error> {
    //打印当前时间

    let update_result = asset_freezes::Entity::update_many()
        .col_expr(
            asset_freezes::Column::FreezeAmount,
            Expr::col(asset_freezes::Column::FreezeAmount).add(amount),
        )
        .col_expr(asset_freezes::Column::UpdatedAt, SimpleExpr::from(Utc::now().timestamp_millis()))
        .filter(
            asset_freezes::Column::TransId
                .eq(trans_id)
                .and(asset_freezes::Column::UserId.eq(user_id))
                .and(asset_freezes::Column::Symbol.eq(symbol)),
        )
        .exec(db)
        .await.expect("");

    println!("Updated {} records", update_result.rows_affected);
    Ok(())
}

pub async fn find_by_symbol(
    db: &DatabaseConnection,
    symbol: &str,
) -> Result<Vec<trade_varieties::Model>, sea_orm::DbErr> {
    let condition = Condition::all().add(trade_varieties::Column::Symbol.eq(symbol));

    // 查询数据库获取 TradeVariety
    let result = trade_varieties::Entity::find()
        .filter(condition)
        .all(db)
        .await?;

    Ok(result)
}

pub async fn validate_order_limit(
    data: &order::Model,
    db: &DatabaseConnection,
) -> Result<(), String> {
    // TODO: 数量检查
    if data.quantity.is_zero() {
        return Err("Quantity cannot be zero".to_string());
    }

    // TODO: 价格检查
    if data.price.is_zero() {
        return Err("Price cannot be zero".to_string());
    }

    // TODO: 对向订单检查，防止自己的买单和卖单成交
    let opposite_side = match data.order_side.as_str() {
        "buy" => "sell",
        "sell" => "buy",
        _ => return Err("Invalid order side".to_string()),
    };

    let condition = Condition::all()
        .add(order::Column::Symbol.eq(data.symbol.clone()))
        .add(order::Column::UserId.eq(data.user_id.clone()))
        .add(order::Column::OrderSide.eq(opposite_side));

    // 查询数据库获取 TradeVariety
    let existing_order = order::Entity::find().filter(condition).all(db).await;

    if !existing_order.unwrap().is_empty() {
        return Err("Opposite order exists for the same user".to_string());
    }

    Ok(())
}
async fn create_market_order_by_amount(
    user_id: &str,
    symbol: &str,
    side: &str,
    amount: Decimal,
) -> Result<order::Model, String> {
    // 实现创建按金额市价订单的逻辑
    unimplemented!()
}

async fn create_market_order_by_qty(
    user_id: &str,
    symbol: &str,
    side: &str,
    quantity: Decimal,
) -> Result<order::Model, String> {
    // 实现创建按数量市价订单的逻辑
    unimplemented!()
}

async fn publish_event(event: EventOrderNew) {
    // 实现发布事件的逻辑
    unimplemented!()
}
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
