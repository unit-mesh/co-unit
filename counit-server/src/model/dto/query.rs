use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SimpleQuery {
    pub q: String,
}
