use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub github: GithubConfig,
    pub leetcode: LeetcodeConfig,
    pub unlock: UnlockConfig,
    pub firewall: FirewallConfig,
}

#[derive(Deserialize)]

pub struct GithubConfig {
    pub token: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct LeetcodeConfig {
    pub username: String,
}

#[derive(Deserialize)]
pub struct UnlockConfig {
    pub duration_minutes: i32,
}

#[derive(Deserialize)]
pub struct FirewallConfig {
    pub tcp_ports: Vec<u16>,
    pub udp_ports: Vec<u16>,
}

pub fn load_config() -> Config {
    let content = std::fs::read_to_string("config.toml").expect("Could not find config.toml");
    let mut config: Config = toml::from_str(&content).expect("config.toml is not formatted correctly");
    config.github.token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
    config
}