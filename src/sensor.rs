use std::fmt::Display;

use schemars::Schema;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigurationError {
  SchemaViolation(String)
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          Self::SchemaViolation(msg) => write!(f, "Configuration violates schema: {msg}")
        }
    }
}

pub trait Sensor {
    fn new() -> Self
    where
        Self: Sized;
    fn get_config_schema() -> Schema
    where
        Self: Sized;
    fn configure(&self, config: &serde_json::Value) -> Result<(), ConfigurationError>;
}
