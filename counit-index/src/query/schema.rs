use std::{borrow::Cow, collections::HashMap};

use qdrant_client::{
    qdrant::{
        point_id::PointIdOptions, PointId,
        RetrievedPoint,
        ScoredPoint, Value, Vectors, vectors::VectorsOptions,
    },
};

pub type Embedding = Vec<f32>;

#[derive(Default, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Payload {
    pub lang: String,
    pub repo_name: String,
    pub repo_ref: String,
    pub relative_path: String,
    pub content_hash: String,
    pub text: String,
    // TODO: for save some in Chinese or other the utf8 char
    // pub origin_text: String,
    pub start_line: u64,
    pub end_line: u64,
    pub start_byte: u64,
    pub end_byte: u64,
    pub branches: Vec<String>,

    #[serde(skip)]
    pub id: Option<String>,
    #[serde(skip)]
    pub embedding: Option<Embedding>,
    #[serde(skip)]
    pub score: Option<f32>,
}

impl PartialEq for Payload {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang
            && self.repo_name == other.repo_name
            && self.repo_ref == other.repo_ref
            && self.relative_path == other.relative_path
            && self.content_hash == other.content_hash
            && self.text == other.text
            && self.start_line == other.start_line
            && self.end_line == other.end_line
            && self.start_byte == other.start_byte
            && self.end_byte == other.end_byte
            && self.branches == other.branches

        // ignoring deserialized fields that will not exist on a newly
        // created payload
    }
}



macro_rules! val_str(($hash:ident, $val:expr) => { serde_json::from_value($hash.remove($val).unwrap()).unwrap() });
macro_rules! val_parse_str(($hash:ident, $val:expr) => {
    serde_json::from_value::<Cow<'_, str>>($hash.remove($val).unwrap())
        .unwrap()
        .parse()
        .unwrap()
});

impl Payload {
    pub fn from_qdrant(orig: ScoredPoint) -> Payload {
        let ScoredPoint {
            id,
            payload,
            score,
            vectors,
            ..
        } = orig;

        parse_payload(id, vectors, payload, score)
    }

    pub fn from_scroll(orig: RetrievedPoint) -> Payload {
        let RetrievedPoint {
            id,
            payload,
            vectors,
            ..
        } = orig;

        parse_payload(id, vectors, payload, 0.0)
    }

    pub(crate) fn into_qdrant(self) -> HashMap<String, Value> {
        HashMap::from([
            ("lang".into(), self.lang.to_ascii_lowercase().into()),
            ("repo_name".into(), self.repo_name.into()),
            ("repo_ref".into(), self.repo_ref.into()),
            ("relative_path".into(), self.relative_path.into()),
            ("content_hash".into(), self.content_hash.into()),
            ("snippet".into(), self.text.into()),
            ("start_line".into(), self.start_line.to_string().into()),
            ("end_line".into(), self.end_line.to_string().into()),
            ("start_byte".into(), self.start_byte.to_string().into()),
            ("end_byte".into(), self.end_byte.to_string().into()),
            ("branches".into(), self.branches.into()),
        ])
    }
}

fn parse_payload(
    id: Option<PointId>,
    vectors: Option<Vectors>,
    payload: HashMap<String, Value>,
    score: f32,
) -> Payload {
    let Some(PointId { point_id_options: Some(PointIdOptions::Uuid(id)) }) = id
        else {
            // unless the db was corrupted/written by someone else,
            // this shouldn't happen
            unreachable!("corrupted db");
        };

    let embedding = match vectors {
        None => None,
        Some(Vectors {
                 vectors_options: Some(VectorsOptions::Vector(v)),
             }) => Some(v.data),
        _ => {
            // this also should probably never happen
            unreachable!("got non-vector value");
        }
    };

    let mut converted = payload
        .into_iter()
        .map(|(key, value)| (key, kind_to_value(value.kind)))
        .collect::<HashMap<String, serde_json::Value>>();

    Payload {
        lang: val_str!(converted, "lang"),
        repo_name: val_str!(converted, "repo_name"),
        repo_ref: val_str!(converted, "repo_ref"),
        relative_path: val_str!(converted, "relative_path"),
        content_hash: val_str!(converted, "content_hash"),
        text: val_str!(converted, "snippet"),
        branches: val_str!(converted, "branches"),
        start_line: val_parse_str!(converted, "start_line"),
        end_line: val_parse_str!(converted, "end_line"),
        start_byte: val_parse_str!(converted, "start_byte"),
        end_byte: val_parse_str!(converted, "end_byte"),

        id: Some(id),
        score: Some(score),
        embedding,
    }
}

fn kind_to_value(kind: Option<qdrant_client::qdrant::value::Kind>) -> serde_json::Value {
    use qdrant_client::qdrant::value::Kind;
    match kind {
        Some(Kind::NullValue(_)) => serde_json::Value::Null,
        Some(Kind::BoolValue(v)) => serde_json::Value::Bool(v),
        Some(Kind::DoubleValue(v)) => {
            serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap())
        }
        Some(Kind::IntegerValue(v)) => serde_json::Value::Number(v.into()),
        Some(Kind::StringValue(v)) => serde_json::Value::String(v),
        Some(Kind::ListValue(v)) => serde_json::Value::Array(
            v.values
                .into_iter()
                .map(|v| kind_to_value(v.kind))
                .collect(),
        ),
        Some(Kind::StructValue(_v)) => todo!(),
        None => serde_json::Value::Null,
    }
}