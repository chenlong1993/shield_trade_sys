# Shield Trade System

基于Actix-web的高性能交易系统
> 一个写着玩的系统，2021年开始，目前整体该在完善中，后续会添加更多功能，还没通过测试，一些还没跑通过测试的代码，有空就写，想起来就写，就是个爱好，想交流的提pr

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

###  main.rs:

应用程序入口，配置 Actix Web 服务器和路由。
###  config.rs:

配置文件，用于加载和管理应用程序的配置参数。
### models/:

存放数据模型，每个模型对应一个文件，如用户模型和交易模型。
### handlers/:

存放请求处理函数，每个模块对应一类 API，如用户相关的处理函数和交易相关的处理函数。
### services/:

存放业务逻辑，每个模块对应一类业务逻辑，如用户服务和交易服务。
### repositories/:

存放数据访问层代码，用于与数据库进行交互。
### errors.rs:

定义应用程序中的错误类型和错误处理。
### utils.rs:

存放一些通用的工具函数。

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

4.详细文档
- [API文档](./docs/api.md)
