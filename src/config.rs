use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(alias = "name")] 
    pub network_name: String,

    pub port: u16,
    pub refresh_time: u64,
    pub servers: HashMap<String, ServerConfig>,
    pub actions: ActionsConfig
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub rcon: Option<RconConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RconConfig {
    pub port: u16,
    pub password: String
}


#[derive(Debug, Deserialize, Clone)]
pub struct ActionsConfig {
    pub app_error: String,
    pub server_off: String,
    pub server_on: String
}

pub fn read_config() -> Result<Config, AppError> {
    // Read and parse the config file
    let config_str = fs::read_to_string("config.toml")
        .map_err(|err| AppError::ConfigError(format!("Failed to read config: {}", err)))?;
    Ok(toml::from_str(&config_str)
        .map_err(|err| AppError::ConfigError(format!("Failed to parse config: {}", err)))?)
}
