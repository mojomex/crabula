use std::fmt::Display;

use schemars::Schema;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigurationError {
    SchemaViolation(String),
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaViolation(msg) => write!(f, "Configuration violates schema: {msg}"),
        }
    }
}

pub trait SensorManifest {
    fn get_config_schema(&self) -> Schema;
}

pub trait Sensor {
    fn new() -> Self
    where
        Self: Sized;

    fn configure(&mut self, config: &serde_json::Value) -> Result<(), ConfigurationError>;
}

type DiagnosticsCallback = fn(Value) -> ();
pub trait Diagnosable {
    fn register_diagnostics_hook(callback: &DiagnosticsCallback);
}
