use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mc_query::status::data::StatusResponse;
use serde::Serialize;

use crate::{
    cmd::{execute_command, CommandType},
    config::Config,
    errors::AppError,
    queries::status::{query_all_servers, query_server, ServerInfo},
};

#[derive(Debug, Serialize)]
pub struct ServerResponse {
    info: ServerInfo,
    status: Option<StatusResponse>,
    error: Option<AppError>,
}

pub async fn get_server(
    State(config): State<Arc<Config>>,
    Path(name): Path<String>,
) -> Result<Json<ServerResponse>, StatusCode> {
    let server_config = config.servers.get(&name).ok_or(StatusCode::NOT_FOUND)?;

    let info = ServerInfo {
        name: name,
        host: server_config.host.clone(),
        port: server_config.port,
    };

    let server_status = query_server(server_config).await;

    match server_status {
        Ok(status) => Ok(Json(ServerResponse {
            info,
            status: Some(status),
            error: None,
        })),
        Err(err) => Ok(Json(ServerResponse {
            info,
            status: None,
            error: Some(err),
        })),
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
