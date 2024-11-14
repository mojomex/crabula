use sensor::ConfigurationError;
use sensors::{get_sensor, SensorNotFoundError};
use serde_json::Value;
use thiserror::Error;

mod backbone;
pub mod sensor;
pub mod sensors;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("Unknown sensor model")]
    SensorNotFoundError(#[from] SensorNotFoundError),
    #[error("Failed to configure sensor: {0}")]
    ConfigurationError(#[from] ConfigurationError),
}

pub fn run(sensor_model: &str, config: Value) -> Result<(), RunError> {
    let mut sensor = get_sensor(sensor_model)?;
    sensor.configure(&config)?;

    Ok(())
}
