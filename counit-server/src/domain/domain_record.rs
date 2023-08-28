#[derive(Debug, serde::Deserialize)]
pub struct DomainRecord {
    native: String,
    english: String,
    abbreviation: String,
    description: String,
}
