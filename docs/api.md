# Trading Engine API Documentation

## Order API

### Create Order

**Endpoint**: POST /orders

**Request Body**:
```json
{
  "symbol": "string",       // 交易对，如BTCUSDT
  "price": "number|null",   // 限价单价格，市价单为null
  "quantity": "number",     // 数量
  "side": "string",         // 买卖方向：buy/sell
  "amount": "number|null"   // 市价单按金额交易时使用
}
```

**Response**:
- Success (200):
```json
{
  "code": 200,
  "data": [
    {
      "trade_id": "uuid",
      "price": "number",
      "quantity": "number",
      "timestamp": "datetime"
    }
  ]
}
```
- Validation Error (400):
```json
{
  "code": 400,
  "message": "string"
}
```
- Internal Error (500):
```json
{
  "code": 500,
  "message": "string"
}
```

### Cancel Order

**Endpoint**: DELETE /orders/{order_id}

**Path Parameters**:
- order_id: string (订单ID)

**Response**:
- Success (200):
```json
{
  "code": 200,
  "data": "Order canceled"
}
```
- Not Found (404):
```json
{
  "code": 404,
  "message": "Order not found"
}
```
- Internal Error (500):
```json
{
  "code": 500,
  "message": "string"
}
```

## Error Codes

| Code | Description |
|------|-------------|
| 400  | 请求参数验证失败 |
| 404  | 订单不存在 |
| 500  | 服务器内部错误 |
