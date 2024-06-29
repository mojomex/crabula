use std::{
    fs::File,
    path::Path,
};

use jsonschema::JSONSchema;
use serde_json::{json, Value};

const CONFIG_DIR: &str = "/home/maximilianschmeller/rubula/config";
const SCHEMA_DIR: &str = "/home/maximilianschmeller/rubula/schema";

fn main() {
    let sensor_model = "OT128";

    let schema = {
        let path = Path::new(SCHEMA_DIR)
            .join(sensor_model.to_owned() + ".json")
            .canonicalize()
            .expect("Valid schema path");
        let file = File::open(&path).expect("Valid file");
        let mut json: Value = serde_json::from_reader(file).expect("Valid JSON");
        json["$id"] = json!(&("file://".to_owned() + &path.to_string_lossy().to_string()));
        JSONSchema::options()
            .with_draft(jsonschema::Draft::Draft7)
            .compile(&json)
            .expect("Valid schema")
    };

    let config: Value = {
        let path = Path::new(CONFIG_DIR).join(sensor_model.to_owned() + ".param.yaml");
        let file = File::open(path).expect("Valid file");
        serde_yaml::from_reader(file).expect("Valid YAML")
    };

    if let Err(errors) = schema.validate(&config) {
        for error in errors {
            println!("Error {} @ {}", error, error.instance_path);
        }
        return;
    }

    let config = &config["/**"]["ros__parameters"];

    // Config is valid
    let sensor_ip = config["sensor_ip"].as_str().expect("Valid sensor IP");
    let host_ip = config["host_ip"].as_str().expect("Valid host IP");
    let data_port = config["data_port"].as_u64().expect("Valid data port");

    println!("Connecting to sensor @ {sensor_ip}:{data_port} from {host_ip}");
}
