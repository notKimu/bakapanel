use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use sysinfo::System;

use super::ApiError;

#[derive(Serialize)]
pub struct SysInfo {
    os_name: Option<String>,
    os_kernel_version: Option<String>,
    cpu_usage: f32,
    cpu_threads: Vec<CpuInfo>,
    ram_max: u64,
    ram_used: u64,
}

#[derive(Serialize)]
pub struct CpuInfo {
    name: String,
    usage: f32,
    frecuency: u64,
}

pub async fn get_status(State(sys): State<Arc<Mutex<System>>>) -> Result<Json<SysInfo>, ApiError> {
    let mut sys = sys.lock().map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error reading system info: {}", err.to_string()),
        )
    })?;

    sys.refresh_memory();
    sys.refresh_cpu_all();

    let cpus: Vec<CpuInfo> = sys
        .cpus()
        .iter()
        .map(|cpu| CpuInfo {
            name: cpu.name().to_owned(),
            usage: cpu.cpu_usage(),
            frecuency: cpu.frequency(),
        })
        .collect();

    Ok(Json(SysInfo {
        os_name: System::name(),
        os_kernel_version: System::kernel_version(),
        cpu_usage: sys.global_cpu_usage(),
        cpu_threads: cpus,
        ram_max: sys.total_memory(),
        ram_used: sys.used_memory(),
    }))
}
