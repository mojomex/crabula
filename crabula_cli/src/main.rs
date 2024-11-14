use std::{fs::File, path::Path, process::exit};

use crabula_core::{run, sensors, RunError};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    verb: Verb,
}

#[derive(Subcommand)]
enum Verb {
    /// List all sensor models
    List,
    /// Generates schema files for all sensor configurations in JSON format
    GenerateSchemas,
    /// Run Crabula for a given sensor model
    Run {
        /// The sensor model to configure Crabula for. List all available models with the list verb
        sensor_model: String,
        /// The file containing the sensor's configuration in YAML format
        config_file: String,
    },
}

fn main() -> Result<(), RunError>{
    let cli = Cli::parse();

    match &cli.verb {
        Verb::List => Ok(list_models()),
        Verb::GenerateSchemas => Ok(generate_schemas()),
        Verb::Run {
            sensor_model,
            config_file,
        } => parse_config_and_run(sensor_model, config_file),
    }
}

fn parse_config_and_run(sensor_model: &str, config_file: &str) -> Result<(), RunError> {
    let open_result = File::open(config_file);
    let Ok(reader) = open_result else {
        println!(
            "Could not open parameter file {}: {}",
            config_file,
            open_result.unwrap_err()
        );
        exit(1);
    };
    let parse_result = serde_yaml::from_reader(reader);
    let Ok(config) = parse_result else {
        println!(
            "Could not parse parameter file {}: {}",
            config_file,
            parse_result.unwrap_err()
        );
        exit(1)
    };

    run(sensor_model, config)
}

fn list_models() {
    println!("The following sensors are supported:");
    for sensor in sensors::SENSORS {
        println!("- {sensor}");
    }
}

fn generate_schemas() {
    for sensor in sensors::SENSORS {
        let schema = sensors::get_schema(sensor).expect("sensor DB listed sensor without schema");
        let file =
            File::create(Path::new(&format!("{sensor}.json"))).expect("file should be creatable");
        serde_json::to_writer_pretty(file, &schema)
            .expect("JSON schema should be serializable to file");
    }
}
