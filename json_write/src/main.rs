use serde::{Deserialize, Serialize};

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
    let article: Article = Article {
        article: String::from("How to work with json in Rust"),
        author: String::from("Mannan"),
        paragraph: vec![
            Paragraph {
                name: String::from("First Sentence"),
            },
            Paragraph {
                name: String::from("Body of the Paragraph"),
            },
            Paragraph {
                name: String::from("Another Body of the Paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is {}", json);
}