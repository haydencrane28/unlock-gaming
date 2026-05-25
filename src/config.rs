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
    pub duration_minutes: u64,
}

#[derive(Deserialize)]
pub struct FirewallConfig {
    pub tcp_ports: Vec<u16>,
    pub udp_ports: Vec<u16>,
}

pub fn load_config() -> Config {
    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let config_path = format!("{}/.config/unlock-gaming/config.toml", home);
    let content = std::fs::read_to_string(config_path).expect("Could not find config.toml");    let mut config: Config = toml::from_str(&content).expect("config.toml is not formatted correctly");
    config.github.token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set");
    config
}