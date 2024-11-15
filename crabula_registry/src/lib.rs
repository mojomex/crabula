use crabula_core::sensor::SensorManifest;
use crabula_hesai_ot128::OT128Manifest;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Sensor model {0} not found")]
pub struct SensorNotFoundError(String);

pub trait Registry {
    fn get_sensor_models(&self) -> &[&str];

    fn get_manifest(&self, model: &str) -> Result<Box<dyn SensorManifest>, SensorNotFoundError>;
}

const SENSORS: &[&str] = &["OT128"];
pub struct StaticRegistry;

impl Registry for StaticRegistry {
    fn get_sensor_models(&self) -> &[&str] {
        SENSORS
    }

    fn get_manifest(&self, model: &str) -> Result<Box<dyn SensorManifest>, SensorNotFoundError> {
        match model {
            "OT128" => Ok(Box::new(OT128Manifest)),
            _ => Err(SensorNotFoundError(model.to_owned())),
        }
    }
}
