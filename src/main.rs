mod config;
mod checks;

use crate::config::load_config;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = load_config();

    let result = checks::github::request_from_github(
        &config.github.username,
        &config.github.token,
    ).await;

    println!("Commited to GitHub today: {}", result);

    let result = checks::leetcode::request_from_leetcode(
        &config.leetcode.username,
    ).await;

    println!("Commited to LeetCode today: {}", result);

}