use reqwest::Client;
use serde_json::json;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN environment variable is not set");

    // List of repositories in the format "owner/repo"
    let repos = vec!["xTwo56/Dev-Dialogue"];

    let client = Client::new();

    for repo in repos {
        let url = format!("https://api.github.com/repos/{}", repo);

        let response = client
            .patch(&url)
            .bearer_auth(&token)
            .header("User-Agent", "Rust-Script")
            .json(&json!({ "private": true }))
            .send()
            .await?;

        if response.status().is_success() {
            println!("Successfully made {} private", repo);
        } else {
            eprintln!("Failed to update {}: {}", repo, response.text().await?);
        }
    }

    Ok(())
}
