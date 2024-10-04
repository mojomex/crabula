mod hesai;

use std::fmt::Display;

use hesai::ot128::OT128;
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

pub static SENSORS : &[&str] = &["OT128"];

pub fn get_schema(sensor_model : &str) -> Result<Schema, SensorNotFoundError> {
  match sensor_model {
    "OT128" => Ok(OT128::get_config_schema()),
    _ => Err(SensorNotFoundError)
  }
}

pub fn get_sensor(sensor_model : &str) -> Result<Box<dyn Sensor>, SensorNotFoundError> {
  match sensor_model {
    "OT128" => Ok(Box::new(OT128::new())),
    _ => Err(SensorNotFoundError)
  }
}
