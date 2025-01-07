# Shield Trade System

基于Actix-web的高性能交易系统

## 模块说明

### 核心模块
- `src/matching/` 交易撮合引擎
  - `matching_engine.rs` 撮合引擎主逻辑
  - `order_book.rs` 订单簿实现
  - `mod.rs` 撮合引擎接口定义

### API模块
- `src/api/` HTTP API接口
  - `matching.rs` 撮合相关API实现
  - `mod.rs` API路由配置

### 类型定义
- `src/types/` 系统类型定义
  - `mod.rs` 订单、交易等核心类型

### 错误处理
- `src/error.rs` 系统错误定义

## 快速开始

1. 安装依赖
```bash
cargo build
```

2. 启动服务
```bash
cargo run
```

3. API文档
- POST /matching/order 提交订单
- DELETE /matching/order/{order_id} 取消订单
- GET /matching/orderbook/{symbol} 获取订单簿
