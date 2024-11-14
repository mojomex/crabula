use std::fmt::Display;

use schemars::Schema;
use thiserror::Error;

use crate::sensor::Sensor;

#[derive(Debug, Error)]
pub struct SensorNotFoundError;

impl Display for SensorNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sensor model not found")
    }
}

pub static SENSORS: &[&str] = &["OT128"];

pub fn get_schema(sensor_model: &str) -> Result<Schema, SensorNotFoundError> {
    Err(SensorNotFoundError)
}

pub fn get_sensor(sensor_model: &str) -> Result<Box<dyn Sensor>, SensorNotFoundError> {
    Err(SensorNotFoundError)
}
