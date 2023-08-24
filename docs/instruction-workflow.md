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

### 2. 语义化搜索

两种方式：

1. HyDE doc
2. 语义解析和搜索

### 3. 生成和合并 chunk

### 4. 总结和生成初步代码

### 5. 根据上下文生成代码

