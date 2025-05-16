use std::sync::{Arc, Mutex};

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::get,
    Router,
};
use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use server::{get_all_servers, get_server};
use status::get_status;
use sysinfo::System;
use tower_http::cors::{Any, CorsLayer};

use crate::{config::Config, errors::AppError};

pub mod server;
pub mod status;

#[derive(RustEmbed, Clone)]
#[folder = "frontend/build/"]
struct Build;

pub async fn start(config: Arc<Config>, system: Arc<Mutex<System>>) -> Result<(), AppError> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let api_router = Router::new()
        .route(
            "/api/servers",
            get(get_all_servers).with_state(config.clone()),
        )
        .route(
            "/api/server/{name}",
            get(get_server).with_state(config.clone()),
        )
        .route("/api/status", get(get_status).with_state(system.clone()));

    let static_files_service = ServeEmbed::<Build>::new();

    let app = Router::new()
        .merge(api_router)
        .fallback_service(static_files_service)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", &config.port))
        .await
        .map_err(|err| AppError::ApiError(format!("Error while binding to port: {}", err)))?;

    println!("Listening at http://127.0.0.1:{}", &config.port);

    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::ApiError(format!("Error while serving the API: {}", err)))?;

    Ok(())
}
