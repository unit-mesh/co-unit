use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DescriptionDsl {
    #[serde(alias = "领域")]
    pub domain: String,
    #[serde(alias = "查询条件")]
    pub query: String,
    #[serde(alias = "假设性文档")]
    pub hypothetical_document: String,
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

    use super::*;

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