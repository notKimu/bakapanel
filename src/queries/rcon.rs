use mc_query::rcon::RconClient;

use crate::{config::RconConfig, errors::AppError};

// TODO: Don't create a client each time a command is sent
pub async fn rcon_command(
    host: &str,
    rcon_config: &RconConfig,
    cmd: &str,
) -> Result<String, AppError> {
    let mut client = RconClient::new(host, rcon_config.port)
        .await
        .map_err(|err| AppError::QueryError(format!("{}", err.to_string())))?;
    client
        .authenticate(&rcon_config.password)
        .await
        .map_err(|err| AppError::QueryError(format!("{}", err.to_string())))?;

    Ok(client
        .run_command(cmd)
        .await
        .map_err(|err| AppError::QueryError(format!("{}", err.to_string())))?)
}
