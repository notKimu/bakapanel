use std::collections::HashMap;

use mc_query::status::data::StatusResponse;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

use crate::{config::ServerConfig, errors::AppError};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub rcon: bool,
}

pub async fn query_server(server_config: &ServerConfig) -> Result<StatusResponse, AppError> {
    Ok(mc_query::status(&server_config.host, server_config.port)
        .await
        .map_err(|err| AppError::QueryError(format!("Error querying sever: {}", err)))?)
}

pub async fn query_all_servers(
    servers: &HashMap<String, ServerConfig>,
) -> JoinSet<(Result<StatusResponse, AppError>, ServerInfo)> {
    let mut set: JoinSet<(Result<StatusResponse, AppError>, ServerInfo)> = JoinSet::new();

    for (name, config) in servers {
        let name_clone = name.clone();
        let config_clone = config.clone();

        set.spawn(async move {
            (
                query_server(&config_clone).await,
                ServerInfo {
                    name: name_clone,
                    host: config_clone.host,
                    port: config_clone.port,
                    rcon: config_clone.rcon.is_some()
                },
            )
        });
    }

    set
}
