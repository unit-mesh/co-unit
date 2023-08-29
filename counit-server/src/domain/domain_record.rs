#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DomainRecord {
    pub(crate) native: String,
    pub(crate) english: String,
    pub(crate) abbreviation: String,
    pub(crate) description: String,
}
