use crate::models::AppConfig;
use std::fs;
use std::path::Path;

pub fn load_config() -> AppConfig {
    let config_path = Path::new("config.json");
    if let Ok(content) = fs::read_to_string(config_path) {
        if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
            return config;
        }
    }
    AppConfig::default()
}

pub fn save_config(config: &AppConfig) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write("config.json", content)?;
    Ok(())
}
