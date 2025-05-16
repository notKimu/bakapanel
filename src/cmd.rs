use std::process::Command;

use crate::{errors::AppError, queries::status::ServerInfo};

pub enum CommandType<'a> {
    AppError(AppError),
    ServerInfo(&'a ServerInfo),
}

pub fn execute_command(cmd_type: CommandType, command_string: &str) -> Result<(), AppError> {
    let command_string = match cmd_type {
        CommandType::AppError(err) => command_string.replace("{ERROR}", &err.to_string()),
        CommandType::ServerInfo(sv_info) => command_string
            .replace("{SERVER_NAME}", &sv_info.name)
            .replace("{SERVER_HOST}", &sv_info.host)
            .replace("{SERVER_PORT}", &sv_info.port.to_string()),
    };

    let args = shlex::split(&command_string).ok_or(AppError::ConfigError(String::from(
        "Failed to parse command string",
    )))?;

    let command_name = &args[0];
    let command_args = &args[1..];

    let output = Command::new(command_name)
        .args(command_args)
        .output()
        .map_err(|err| AppError::TaskError(format!("Error executing a command: {}", err)))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(AppError::TaskError(format!(
            "Command execution failed: {}",
            stderr
        )))
    }
}
