use reqwest;
use serde_json::Value;

pub async fn check_ollama_status(url: &str) -> bool {
    let client = reqwest::Client::new();
    match client.get(format!("{}/api/tags", url)).send().await {
        Ok(res) => res.status().is_success(),
        Err(_) => false,
    }
}
