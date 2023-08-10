use std::path::Path;
use std::sync::Arc;

use serde_json::json;

use crate::semantic::configuration::Configuration;
use crate::semantic::literal::Literal;
use crate::semantic::semantic::Semantic;
use crate::semantic::semantic_query::SemanticQuery;

pub mod semantic;

#[tokio::main]
async fn main() {
    let model_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("model");
    println!("{}", model_dir.to_str().unwrap());

    let config = serde_json::from_value::<Configuration>(json!({

        }))
        .unwrap();

    let sm = Semantic::initialize(&model_dir, "http://127.0.0.1:6334", Arc::new(config))
        .await
        .unwrap();


    sm.embed(r#"// PUT /teams/{teamId}/objectives/{objectiveId}
// ObjectiveDTO: { "name": string, "description": string, "type": string }
// ObjectiveResponse: { "id": string, "name": string, "description": string, "type": string }
"#);

    sm.embed(r#"// POST /employees/{employeeId}/okrs
// OKRDTO: { "objectiveId": string, "description": string, "target": string, "progress": string, "startDate": string, "endDate": string }
// OKRResponse: { "id": string, "objectiveId": string, "description": string, "target": string, "progress": string, "startDate": string, "endDate": string }
"#);

    sm.embed(r#"// PUT /employees/{employeeId}/okrs/{okrId}
// OKRDTO: { "objectiveId": string, "description": string, "target": string, "progress": string, "startDate": string, "endDate": string }
// OKRResponse: { "id": string, "objectiveId": string, "description": string, "target": string, "progress": string, "startDate": string, "endDate": string }
"#);

    let query = SemanticQuery {
        target: Some(Literal::Plain("employeeId".into())),
        repos: Default::default(),
        paths: Default::default(),
        langs: Default::default(),
        branch: Default::default(),
    };

    let result = sm.search(&query, 30, 0, 0.0, true).await.unwrap();
    println!("{:?}", result);
}
