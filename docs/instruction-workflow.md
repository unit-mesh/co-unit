# Instruction Workflow

## API 对接

用户指令：

```
/counit 帮我对接一个 创建支付宝订单 的接口
```

### 1. 识别和并翻译用户指令

转换成 DSL

```json
{
  "types": [
    ""
  ],
  "query": "post alipay order"
}
```

prompt 设计：

```markdown
Your job is to translate/transpile user's question relative to codebase.

1. You MUST translate user's question into a DSL query.
2. DON'T translate user's question into a code snippet.
3. `query` is a reference to the document that you think is the answer to the question.   
4. `hypothetical_document` is a example of the document that you think is the answer to the question.   
5. DON'T explain the DSL.

For example:

Q: 帮我接入 统一收单交易撤销 的接口
A: { "domain": "payment", "query": "cancel Unified Acquiring Transaction", "hypothetical_document": 'POST /api/alipay/trade/cancel {"action":"close","gmt_refund_pay":"officia nostrud est","out_trade_no":"6823789339978248","refund_settlement_id":"2018101610032004620239146945","retry_flag":"N","trade_no":"2013112011001004330000121536"}'" }

Q: 如何查询职得(jobworth)工作证信息？
A: { "domain": "customer", "query": "query jobworth work permit information", "hypothetical_document": 'GET /api/customer/jobworth/info/query?user_name=张三' }

Q: 因公付(enterprisepay)更新员工资金协议
A: { "domain": "fund", "query": "update employee fund agreement for enterprisepay", "hypothetical_document": 'PUT /api/fund/enterprisepay/sign {"employee_id": "12345", "agreement_type": "fund", "update_fields": {"bank_account": "987654321", "amount": 1500.00}}' }

Q: how to generate payment QR code？  
A:

```

### 2. 语义化搜索

两种方式：

1. HyDE doc
2. 语义解析和搜索

### 3. 生成和合并 chunk

### 4. 总结和生成初步代码

### 5. 根据上下文生成代码

