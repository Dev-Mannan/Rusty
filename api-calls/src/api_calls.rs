use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize; // Import Deserialize trait from serde

#[derive(Debug, Deserialize)]
pub struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let owner = "rust-lang-nursery";
    let repo = "rust-cookbook";
    // Correctly use the format! macro with placeholders for both owner and repo
    let request_url = format!("https://api.github.com/repos/{}/{}/stargazers", owner, repo);

    println!("{}", request_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "rust web api client demo")
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
