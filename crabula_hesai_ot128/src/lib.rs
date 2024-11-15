use jsonschema::JSONSchema;
use num_derive::{FromPrimitive, ToPrimitive};
use schemars::{schema_for, JsonSchema, Schema};
use serde::{Deserialize, Serialize};

use crabula_core::sensor::{ConfigurationError, Sensor, SensorManifest};
use std::net::Ipv4Addr;

mod packet;

pub struct OT128;
pub struct OT128Manifest;

impl SensorManifest for OT128Manifest {
    fn get_config_schema(&self) -> Schema {
        schema_for!(Config)
    }
}

impl Sensor for OT128 {
    fn new() -> OT128 {
        OT128 {}
    }

    fn configure(&mut self, config: &serde_json::Value) -> Result<(), ConfigurationError> {
        let schema = JSONSchema::compile(&OT128Manifest.get_config_schema().to_value())
            .expect("config schema should be valid");

        let validation_result = schema.validate(config);
        if let Err(validation_errors) = validation_result {
            let mut error_msg =
                "the following violations of the parameter schema occured: ".to_owned();
            for e in validation_errors {
                error_msg.push_str("\n- ");
                error_msg.push_str(&e.to_string());
            }
            return Err(ConfigurationError::SchemaViolation(error_msg));
        };

        let config: Config = serde_json::from_value(config.clone())
            .expect("validated config JSON should be convertible to config type");

        Ok(())
    }
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct Config {
    host_ip: Ipv4Addr,
    sensor_ip: Ipv4Addr,
    data_port: u16,
    #[schemars(extend("key" = "value"))]
    frame_rate_hz: u8,
    #[validate(range(min = 0, max = 360))]
    fov_start_deg: f32,
    #[validate(range(min = 0, max = 360))]
    fov_end_deg: f32,
    #[validate(range(min = 0, max = 360))]
    sync_angle_deg: f32,
    #[validate(range(min = 0, max = 360))]
    cut_angle_deg: f32,
    return_mode: ReturnMode,
    ptp_profile: PtpProfile,
}

/*
0x33 — First
0x37 — Strongest
0x38 — Last
0x39 — Last and Strongest
0x3B — Last and First
0x3C — First and Strongest
 */

#[derive(Debug, FromPrimitive, ToPrimitive, JsonSchema, Serialize, Deserialize)]
enum ReturnMode {
    First = 0x33,
    Strongest = 0x37,
    Last = 0x38,
    LastStrongest = 0x39,
    LastFirst = 0x3B,
    FirstStrongest = 0x3C,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
enum PtpProfile {
    IEEE1588v2 {
        transport: IEEE1588v2Transport,
        domain: u8,
    },
    IEEE8021AS {
        domain: u8,
        network_supports_tsn: bool,
    },
    AUTOMOTIVE {
        domain: u8,
        network_supports_tsn: bool,
    },
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
enum IEEE1588v2Transport {
    L2,
    UDPv4,
    UDPv6,
}
