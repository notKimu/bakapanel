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
    pub actions: ActionsConfig,
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
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActionsConfig {
    pub app_error: String,
    pub server_off: String,
    pub server_on: String,
}

static CONFIG_FILENAME: &str = "config.toml";
static DEFAULT_CONFIG: &str = r#"# Name of your network
name = "Network"

# Port for the API to listen to
port = 3000

# How often should the query service be executed
refresh_time = 30

# List of your servers
[servers]

[servers.Localhost]
    host = "localhost"
    port = 25565

[servers.Localhost2]
    host = "localhost"
    port = 25566
    rcon = { port = 25575, password = "" }

# Commands to be ran on different events
[actions]
# Placeholders: {ERROR}
app_error = ""

# Placeholders: {SERVER_NAME} {SERVER_HOST} {SERVER_PORT}
server_off = ""

# Placeholders: {SERVER_NAME} {SERVER_HOST} {SERVER_PORT}
server_on = ""
"#;

pub fn read_config() -> Result<Config, AppError> {
    let exists_config = fs::exists(CONFIG_FILENAME)
        .map_err(|err| AppError::ConfigError(format!("Failed to check the config: {}", err)))?;

    if exists_config {
        let config_str = fs::read_to_string(CONFIG_FILENAME)
            .map_err(|err| AppError::ConfigError(format!("Failed to read the config: {}", err)))?;
        Ok(toml::from_str(&config_str)
            .map_err(|err| AppError::ConfigError(format!("Failed to parse config: {}", err)))?)
    } else {
        fs::write(CONFIG_FILENAME, DEFAULT_CONFIG).map_err(|err| {
            AppError::ConfigError(format!(
                "Failed to write the default config to `./{}`: {}",
                CONFIG_FILENAME, err
            ))
        })?;

        Ok(toml::from_str(DEFAULT_CONFIG).map_err(|err| {
            AppError::ConfigError(format!("Failed to parse the default config, wtf man: {}", err))
        })?)
    }
}
