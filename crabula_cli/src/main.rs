use std::{fs::File, path::Path, process::exit};

use crabula_core::run;

use clap::{Parser, Subcommand};
use crabula_registry::{Registry, SensorNotFoundError, StaticRegistry};

use anyhow::Result;

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    let registry = StaticRegistry;

    match &cli.verb {
        Verb::List => Ok(list_models(&registry)),
        Verb::GenerateSchemas => generate_schemas(&registry),
        Verb::Run {
            sensor_model,
            config_file,
        } => parse_config_and_run(&registry, sensor_model, config_file),
    }
}

fn parse_config_and_run(
    registry: &dyn Registry,
    sensor_model: &str,
    config_file: &str,
) -> Result<()> {
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

    run(sensor_model, config)?;
    Ok(())
}

fn list_models(registry: &dyn Registry) {
    println!("The following sensors are supported:");
    for model in registry.get_sensor_models() {
        println!("- {model}");
    }
}

fn generate_schemas(registry: &dyn Registry) -> Result<()> {
    for model in registry.get_sensor_models() {
        let manifest = registry.get_manifest(model)?;
        let schema = (*manifest).get_config_schema();
        let file = File::create(Path::new(&format!("{model}.json")))?;
        serde_json::to_writer_pretty(file, &schema)?;
    }

    Ok(())
}
