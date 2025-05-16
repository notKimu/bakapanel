use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::time::sleep;

use crate::{
    cmd::{execute_command, CommandType},
    errors::AppError,
    queries::status::query_all_servers,
    Config,
};

pub type ServerStatusList = Mutex<HashMap<String, ServerStatus>>;

#[derive(Debug)]
pub enum ServerStatus {
    Online,
    Offline,
}

pub async fn query_service(config: Arc<Config>, server_states: ServerStatusList) {
    loop {
        let mut set = query_all_servers(&config.servers).await;

        while let Some(res) = set.join_next().await {
            match res {
                // Success joining the tasks
                Ok((sv_result, info)) => match server_states.lock() {
                    Ok(mut server_states_guard) => {
                        let current_status = server_states_guard.get(&info.name);

                        let change = match (sv_result, current_status) {
                            (Ok(_), None) => Some(ServerStatus::Online),
                            (Ok(_), Some(ServerStatus::Offline)) => Some(ServerStatus::Online),
                            (Err(_), None) => Some(ServerStatus::Offline),
                            (Err(_), Some(ServerStatus::Online)) => Some(ServerStatus::Offline),
                            _ => None,
                        };

                        if let Some(new_status) = change {
                            execute_command(
                                CommandType::ServerInfo(&info),
                                match new_status {
                                    ServerStatus::Online => &config.actions.server_on,
                                    ServerStatus::Offline => &config.actions.server_off,
                                },
                            )
                            .ok();
                            server_states_guard.insert(info.name.clone(), new_status);
                        }
                    }
                    Err(err) => {
                        execute_command(
                            CommandType::AppError(AppError::TaskError(err.to_string())),
                            &config.actions.app_error,
                        )
                        .ok();
                    }
                },
                // Handle the error when joining a task
                Err(err) => {
                    execute_command(
                        CommandType::AppError(AppError::TaskError(err.to_string())),
                        &config.actions.app_error,
                    )
                    .ok();
                }
            }
        }

        // Wait for the next iteration
        sleep(Duration::from_secs(config.refresh_time)).await;
    }
}
