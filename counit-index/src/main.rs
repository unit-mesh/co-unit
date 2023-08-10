use std::path::Path;
use std::sync::Arc;

use serde_json::json;

use crate::semantic::configuration::Configuration;
use crate::semantic::literal::Literal;
use crate::semantic::semantic::Semantic;
use crate::semantic::semantic_query::SemanticQuery;

pub mod semantic;
pub mod cache;

#[tokio::main]
async fn main() {
    let model_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("model");

    let config = serde_json::from_value::<Configuration>(json!({

        }))
        .unwrap();

    let sm = Semantic::initialize(&model_dir, "http://127.0.0.1:6334", Arc::new(config))
        .await
        .unwrap();


    sm.insert_points_for_buffer("unit-mesh", "", "../../", r#"作为一个员工，我希望能够创建和更新我的 OKR，以便将我的目标对齐到团队和公司的目标上。
"#).await;

    sm.insert_points_for_buffer("unit-mesh", "", "../../", r#"作为一个团队负责人，我希望能够分配 OKR 给每个员工，并设置期限和优先级，以便能够跟踪整个团队的进展情况。
"#).await;

    sm.insert_points_for_buffer("unit-mesh", "", "../../", r#"作为一个员工，我希望能够及时更新我的 OKR 进展情况，并向团队报告我的进展情况，以便能够保持团队的透明度和协同性。
"#).await;

    let query = SemanticQuery {
        target: Some(Literal::Plain("员工 创建和更新 OKR".into())),
        repos: Default::default(),
        paths: Default::default(),
        langs: Default::default(),
        branch: Default::default(),
    };

    let result = sm.search(&query, 30, 0, 0.3, true).await.unwrap();
    for load in result {
        println!("{:?}", load.text);
    }
}
