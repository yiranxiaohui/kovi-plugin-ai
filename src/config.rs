use std::sync::Arc;
use kovi::log::debug;
use kovi::RuntimeBot;
use kovi::utils::load_toml_data;
use serde::{Deserialize, Serialize};
use toml::toml;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub kind: Kind,
    pub model: String,
    pub api_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Kind {
    Gemini
}

pub fn read_config(bot: Arc<RuntimeBot>) -> Config {
    let data_path = bot.get_data_path();
    let config_toml_path = data_path.join("config.toml");
    let default_config = toml! {
        kind = "Gemini"
        model = "gemini-3-pro-preview"
        api_key = "${API_KEY}"
    };
    let config = load_toml_data(default_config, config_toml_path).unwrap();
    debug!("{}", config.to_string());
    let config: Config = toml::from_str(&config.to_string()).unwrap();
    config
}