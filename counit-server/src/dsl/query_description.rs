use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExplainQuery {
    #[serde(alias = "领域")]
    pub domain: &'static str,
    #[serde(alias = "查询条件")]
    pub query: &'static str,
    #[serde(alias = "假设性文档")]
    pub hypothetical_document: &'static str,
}

impl Display for ExplainQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap();
        write!(f, "{}", json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QAExample {
    pub question: &'static str,
    pub answer: ExplainQuery,
}

impl QAExample {
    fn examples() -> Vec<QAExample> {
        vec![
            QAExample {
                question: "帮我接入统一收单交易撤销的接口",
                answer: ExplainQuery {
                    domain: "payment",
                    query: "cancel Unified Acquiring Transaction",
                    hypothetical_document: "POST /api/alipay/trade/cancel {\"action\":\"close\",\"gmt_refund_pay\":\"officia nostrud est\",\"out_trade_no\":\"6823789339978248\",\"refund_settlement_id\":\"2018101610032004620239146945\",\"retry_flag\":\"N\",\"trade_no\":\"2013112011001004330000121536\"}",
                },
            },
            QAExample {
                question: "如何查询职得(jobworth)工作证信息？",
                answer: ExplainQuery {
                    domain: "customer",
                    query: "query jobworth work permit information",
                    hypothetical_document: "GET /api/customer/jobworth/info/query?user_name=张三",
                },
            },
            QAExample {
                question: "因公付(enterprisepay)更新员工资金协议",
                answer: ExplainQuery {
                    domain: "fund",
                    query: "update employee fund agreement for enterprisepay",
                    hypothetical_document: "PUT /api/fund/enterprisepay/sign {\"employee_id\": \"12345\", \"agreement_type\": \"fund\", \"update_fields\": {\"bank_account\": \"987654321\", \"amount\": 1500.00}}",
                },
            },
        ]
    }

    pub fn prompt(query: &str) -> String {
        let mut prompt = String::new();
        prompt += r#"Your job is to translate/transpile user's question relative to codebase.

1. You MUST translate user's question into a DSL query.
2. DON'T translate user's question into a code snippet.
3. `query` is a reference to the document that you think is the answer to the question.
4. `hypothetical_document` is a example of the document that you think is the answer to the question.
5. DON'T explain the DSL.

For example:

"#;

        for example in Self::examples() {
            prompt.push_str(&format!("Q: {}\n", example.question));
            prompt.push_str(&format!("A: {}\n", example.answer));
            prompt.push_str(&format!("\n"));
        }

        prompt.push_str(&format!("Q: {}\n", query));
        prompt.push_str(&format!("A:"));

        prompt
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
        let config: ExplainQuery = serde_json::from_str(config).unwrap();
        assert_eq!(config.domain, "API");
    }

    #[test]
    fn serialization_from_json_en() {
        let config = r#"{
    "domain": "API",
    "query": "API",
    "hypothetical_document": "API"
}"#;
        let config: ExplainQuery = serde_json::from_str(config).unwrap();
        assert_eq!(config.domain, "API");
    }

    #[test]
    fn prompt_sample() {
        let prompt = QAExample::prompt("帮我接入统一收单交易撤销的接口");
        println!("{}", prompt);
    }
}