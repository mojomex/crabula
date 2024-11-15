use serde_json::Value;
use thiserror::Error;

mod backbone;
pub mod sensor;

#[derive(Debug, Error)]
#[error("Error while running")]
pub struct RunError;

pub fn run(sensor_model: &str, config: Value) -> Result<(), RunError> {
    Ok(())
}
