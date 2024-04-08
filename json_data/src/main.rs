use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"{"article":"The cat was black","author":"Mannan","paragraph":[{"name":"Steal like an artist"},{"name":"Black jack"}]}"#;

    let parsed: Article = read_json_typed(json);

    println!("\n\n The name of the first paragraph is : {}", parsed.paragraph[1].name);
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    parsed
}
