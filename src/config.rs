use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
    pub lua: Lua,
    pub ui: Ui,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub refresh_rate: u64,
}

#[derive(Debug, Deserialize)]
pub struct Lua {
    pub rules_dir: String,
}

#[derive(Debug, Deserialize)]
pub struct Ui {
    pub show_processes: bool,
}

pub fn load_config() -> Config {
    let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
    path.push("sysoracle/config.toml");

    let content = fs::read_to_string(path)
        .expect("Failed to read config.toml");

    toml::from_str(&content)
        .expect("Invalid config.toml")
}
