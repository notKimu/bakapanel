mod api;
mod cmd;
mod config;
mod errors;
mod queries;
mod service;

use config::{read_config, Config};
use errors::AppError;
use service::{query_service, ServerStatusList};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};
use sysinfo::System;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config = read_config()?;

    // System information
    let mut sys = System::new_all();
    sys.refresh_all();

    // Shared variables
    let shared_config_state = Arc::new(config);
    let shared_sytem_state = Arc::new(Mutex::new(sys));

    // Query service loop
    let config_clone = shared_config_state.clone();
    let server_states: ServerStatusList = Mutex::new(HashMap::new());
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async { query_service(config_clone, server_states).await });
    });

    // API
    api::start(shared_config_state, shared_sytem_state).await?;

    Ok(())
}
