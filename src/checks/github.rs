use serde::{Deserialize};
use chrono::{Local};

#[derive(Debug, Deserialize)]
struct GitHubEvent {
    #[serde(rename="type")]
    event_type: String,
    created_at: String,
}

pub async fn request_from_github(username: &str, token: &str) -> bool {
    let url = format!("https://api.github.com/users/{username}/events");

    let dt = Local::now().format("%Y-%m-%d");

    let client = reqwest::Client::new();    
    let response = client.get(url).header("Authorization", format!("token {}", token)).header("User-Agent", "unlock-gaming").send().await;

    match response {
        Err(_) => {
            println!("Failed to get a response.");
            false
        }
        Ok(value) => {
            let result: Vec<GitHubEvent> = value.json().await.unwrap_or_default();

            result.iter().any(|event| {
                event.event_type == "PushEvent" 
                && event.created_at.starts_with(&dt.to_string())
            })
        }
    }
}