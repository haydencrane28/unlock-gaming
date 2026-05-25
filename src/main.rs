mod config;
mod checks;
mod firewall;

use crate::config::load_config;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    dotenv::from_path(
        format!("{}/.config/unlock-gaming/.env", std::env::var("HOME").unwrap())
    ).ok();
    let config = load_config();

    let github_result = checks::github::request_from_github(
        &config.github.username,
        &config.github.token,
    ).await;

    let leetcode_result = checks::leetcode::request_from_leetcode(
        &config.leetcode.username,
    ).await;

    if github_result || leetcode_result {
        if github_result {
            println!("GitHub commit found!");
        }
        if leetcode_result {
            println!("LeetCode submission found!");
        }
        firewall::nftables::unblock();
        let duration = Duration::from_mins(config.unlock.duration_minutes);
        println!("🔓 Gaming unlocked for {} minutes", config.unlock.duration_minutes);
        thread::sleep(duration);
        println!("🔒 Time's up, blocking gaming again");
        firewall::nftables::block(&config.firewall);
    } else {
        println!("You have not made GitHub commit or LeetCode submission today!");
    }

}