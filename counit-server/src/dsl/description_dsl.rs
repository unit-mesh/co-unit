use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DescriptionDsl {
    #[serde(alias = "领域")]
    pub domain: String,
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization_from_json() {
        let config = r#"{
  "领域": "API"
}"#;
        let config: DescriptionDsl = serde_json::from_str(config).unwrap();
        assert_eq!(config.domain, "API");
    }
}