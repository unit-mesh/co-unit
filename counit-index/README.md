# CoUnit Index

Our codebase based on Bloop's [bleep](https://github.com/BloopAI/bloop/tree/main/server/bleep)

We modified the codebase to support our use case.

- reorganized the codebase to make it more modular.
- add support for query by API
- add Chinese / i18n translation support
- add support for multiple data sources


## Differences from Bloop

```toml
# i18n
stardict_wrapper = "0.0.5"
```

translation:

```rust
#[tokio::main]
async fn main() {
    let dict_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("dict");
    let mut dict = WrapperDict::init(dict_dir);

    let jieba = Jieba::new();
    let sentence = translate_sentence(&mut dict, &jieba, first);

    sm.insert_points_for_buffer("unit-mesh", "", "../../", &sentence).await;
}

fn translate_sentence(mut dict: &mut WrapperDict, jieba: &Jieba, sentence: &str) -> String {
    let words: Vec<&str> = jieba.cut(sentence, false);
    let mut result: String = "".to_string();

    for word in words {
        // skip space
        if word == " " {
            continue;
        }

        let trim_word = word.trim();
        if let Some(translate) = dict.translate(trim_word) {
            result = result + &translate + " ";
        } else {
            result = result + trim_word + " ";
        }
    }

    result.to_string()
}

```