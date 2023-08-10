use std::path::Path;
use std::sync::Arc;
use jieba_rs::Jieba;

use serde_json::json;
use stardict_wrapper::WrapperDict;

use crate::semantic::configuration::Configuration;
use crate::semantic::literal::Literal;
use crate::semantic::semantic::Semantic;
use crate::semantic::semantic_query::SemanticQuery;

pub mod semantic;
pub mod cache;

#[tokio::main]
async fn main() {
    let model_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("model");
    let dict_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("dict");

    let mut dict = WrapperDict::init(dict_dir);


    let config = serde_json::from_value::<Configuration>(json!({

        }))
        .unwrap();

    let sm = Semantic::initialize(&model_dir, "http://127.0.0.1:6334", Arc::new(config))
        .await
        .unwrap();

    let jieba = Jieba::new();

    let first = r#"作为一个员工，我希望能够创建和更新我的 OKR，以便将我的目标对齐到团队和公司的目标上。    "#;
    let sentence = translate_sentence(&mut dict, &jieba, first);

    sm.insert_points_for_buffer("unit-mesh", "", "../../", &sentence).await;

    let second = r#"作为一个团队负责人，我希望能够分配 OKR 给每个员工，并设置期限和优先级，以便能够跟踪整个团队的进展情况。"#;
    let sentence2 = translate_sentence(&mut dict, &jieba, second);

    sm.insert_points_for_buffer("unit-mesh", "", "../../", &sentence2).await;

    let third = r#"作为一个员工，我希望能够及时更新我的 OKR 进展情况，并向团队报告我的进展情况，以便能够保持团队的透明度和协同性。
"#;
    let sentence3 = translate_sentence(&mut dict, &jieba, third);

    sm.insert_points_for_buffer("unit-mesh", "", "../../", &sentence3).await;

    let query_str = "员工创建 OKR";
    let result = translate_sentence(&mut dict, &jieba, query_str);
    println!("{}", result);

    let query = SemanticQuery {
        target: Some(Literal::Plain(result.into())),
        repos: Default::default(),
        paths: Default::default(),
        langs: Default::default(),
        branch: Default::default(),
    };

    let result = sm.search(&query, 30, 0, 0.0, true).await.unwrap();
    for load in result {
        println!("{:?}", load.text);
    }
}

fn translate_sentence(mut dict: &mut WrapperDict, jieba: &Jieba, sentence: &str) -> String {
    let words: Vec<&str> = jieba.cut(sentence, false);
    let mut result: String = "".to_string();

    for word in words {
        if let Some(translate) = dict.translate(word) {
            result = result + &translate + " ";
        } else {
            result = result + word + " ";
        }
    }

    result.to_string()
}
