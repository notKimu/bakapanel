use std::sync::Arc;

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    Json,
};
use mc_query::status::data::StatusResponse;
use serde::{Deserialize, Serialize};

use crate::{
    cmd::{execute_command, CommandType},
    config::Config,
    errors::AppError,
    queries::{
        rcon::rcon_command,
        status::{query_all_servers, query_server, ServerInfo},
    },
};

use super::ApiError;

#[derive(Debug, Serialize)]
pub struct ServerResponse {
    info: ServerInfo,
    status: Option<StatusResponse>,
    error: Option<AppError>,
}

pub async fn get_server(
    State(config): State<Arc<Config>>,
    Path(name): Path<String>,
) -> Result<Json<ServerResponse>, ApiError> {
    let server_config = config.servers.get(&name).ok_or((
        StatusCode::NOT_FOUND,
        format!("The server `{}` does not exist", name),
    ))?;

    let info = ServerInfo {
        name: name,
        host: server_config.host.clone(),
        port: server_config.port,
        rcon: server_config.rcon.is_some(),
    };

    let server_status = query_server(server_config).await;

    match server_status {
        Ok(status) => Ok(Json(ServerResponse {
            info,
            status: Some(status),
            error: None,
        })),
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}

pub async fn get_all_servers(State(config): State<Arc<Config>>) -> Json<Vec<ServerResponse>> {
    let mut set = query_all_servers(&config.servers).await;

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        match res {
            Ok((query, cfg)) => match query {
                Ok(data) => {
                    results.push(ServerResponse {
                        info: cfg,
                        status: Some(data),
                        error: None,
                    });
                }
                Err(err) => {
                    results.push(ServerResponse {
                        info: cfg,
                        status: None,
                        error: Some(err),
                    });
                }
            },
            Err(err) => {
                execute_command(
                    CommandType::AppError(&AppError::TaskError(err.to_string())),
                    &config.actions.app_error,
                )
                .await
                .ok();
            }
        }
    }

    Json(results)
}

#[derive(Deserialize)]
pub struct RconCmd {
    cmd: String,
}
pub async fn send_rcon_command<'a>(
    State(config): State<Arc<Config>>,
    Path(name): Path<String>,
    extract::Json(payload): extract::Json<RconCmd>,
) -> Result<String, ApiError> {
    if payload.cmd.len() < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Command length must be equal or greater than 1".to_owned(),
        ));
    }

    let server_config = config.servers.get(&name).ok_or((
        StatusCode::NOT_FOUND,
        format!("The server `{}` does not exist", name),
    ))?;
    let rcon = server_config.rcon.as_ref().ok_or((
        StatusCode::BAD_REQUEST,
        format!("The server `{}` does not have a RCON configuration", name),
    ))?;

    Ok(rcon_command(&server_config.host, rcon, &payload.cmd)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?)
}
