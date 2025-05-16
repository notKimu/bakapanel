use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppError {
    ApiError(String),
    ConfigError(String),
    QueryError(String),
    ParseError(String),
    TaskError(String), 
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ApiError(msg) => write!(f, "Api error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            AppError::QueryError(msg) => write!(f, "Error querying server: {}", msg),
            AppError::ParseError(err) => write!(f, "Parse error: {}", err),
            AppError::TaskError(err) => write!(f, "Task failed to complete: {}", err),
        }
    }
}