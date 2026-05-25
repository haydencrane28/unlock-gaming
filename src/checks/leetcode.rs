use serde::{Deserialize};
use chrono::{Local, DateTime};

#[derive(Debug, Deserialize)]
struct LeetCodeEvent {
    timestamp: String,
}

#[derive(Default, Debug, Deserialize)]
struct Data {
    #[serde(rename = "recentAcSubmissionList")]
    recent_ac_submission_list: Vec<LeetCodeEvent>,
}

#[derive(Default, Debug, Deserialize)]
struct LeetCodeResponse {
    data: Data,
}


pub async fn request_from_leetcode(username: &str) -> bool {
    let url = format!("https://leetcode.com/graphql");

    let dt = Local::now();

    let client = reqwest::Client::new();    
    let response = client.post(url)
        .json(&serde_json::json!({"query": format!("{{ recentAcSubmissionList(username: \"{}\", limit: 10) {{ title timestamp }} }}", username)}))
        .header("Content-Type", "application/json").send().await;

    match response {
        Err(_) => {
            println!("Failed to get a response.");
            false
        }
        Ok(value) => {
            let result: LeetCodeResponse = value.json().await.unwrap_or_default();

            result.data.recent_ac_submission_list.iter().any(|event| {
                let parsed_timestamp = DateTime::from_timestamp(event.timestamp.parse::<i64>().unwrap(), 0).unwrap().with_timezone(&Local);
                
                parsed_timestamp.date_naive() == dt.date_naive()
            })
        }
    }
}