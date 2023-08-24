use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DescriptionDsl {
    #[serde(alias = "领域")]
    pub domain: &'static str,
    #[serde(alias = "查询条件")]
    pub query: &'static str,
    #[serde(alias = "假设性文档")]
    pub hypothetical_document: &'static str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DescExample {
    pub question: &'static str,
    pub answer: DescriptionDsl,
}

impl DescExample {
    pub fn examples() -> Vec<DescExample> {
        vec![
            DescExample {
                question: "帮我接入统一收单交易撤销的接口",
                answer: DescriptionDsl {
                    domain: "payment",
                    query: "cancel Unified Acquiring Transaction",
                    hypothetical_document: "POST /api/alipay/trade/cancel {\"action\":\"close\",\"gmt_refund_pay\":\"officia nostrud est\",\"out_trade_no\":\"6823789339978248\",\"refund_settlement_id\":\"2018101610032004620239146945\",\"retry_flag\":\"N\",\"trade_no\":\"2013112011001004330000121536\"}",
                },
            },
            DescExample {
                question: "如何查询职得(jobworth)工作证信息？",
                answer: DescriptionDsl {
                    domain: "customer",
                    query: "query jobworth work permit information",
                    hypothetical_document: "GET /api/customer/jobworth/info/query?user_name=张三",
                },
            },
            DescExample {
                question: "因公付(enterprisepay)更新员工资金协议",
                answer: DescriptionDsl {
                    domain: "fund",
                    query: "update employee fund agreement for enterprisepay",
                    hypothetical_document: "PUT /api/fund/enterprisepay/sign {\"employee_id\": \"12345\", \"agreement_type\": \"fund\", \"update_fields\": {\"bank_account\": \"987654321\", \"amount\": 1500.00}}",
                },
            },
        ]
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_from_json_zh_cn() {
        let config = r#"{
  "领域": "API",
  "查询条件": "API",
  "假设性文档": "API"
}"#;
        let config: DescriptionDsl = serde_json::from_str(config).unwrap();
        assert_eq!(config.domain, "API");
    }

    #[test]
    fn serialization_from_json_en() {
        let config = r#"{
    "domain": "API",
    "query": "API",
    "hypothetical_document": "API"
}"#;
        let config: DescriptionDsl = serde_json::from_str(config).unwrap();
        assert_eq!(config.domain, "API");
    }
}