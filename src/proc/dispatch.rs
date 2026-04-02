use std::time::Instant;

use clap::{Parser, Subcommand};

use thiserror::Error;

use crate::checksum::checksum_map::{ChecksumMap, ChecksumMapError};
use crate::checksum::generate::generate_checksums;

use crate::format;
use crate::format::path_manager::nba_checksum_file;

use crate::ml::model::Model;
use crate::ml::models::registration::Registration;

use crate::proc::forecast::{forecast_nba, ForecastError};
use crate::proc::historian::{annotate_nba, chronicle_nba, observe_nba};
use crate::proc::refresher::update_source_data;

use crate::stats::chronology::{Chronology, ChronologyError};

#[derive(Parser)]
#[command(name = "warheads")]
#[command(about = "ML models on NBA data")]
#[command(
    long_about = "This program provides a framework to generate and train ML models on NBA data.\nIt requires network access and will make minimal requests to the NBA API."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Sync,
    Checksums {
        #[command(subcommand)]
        action: ChecksumCommand,
    },
    Train {
        model_name: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    #[command(name = "eval")]
    Evaluate {
        model_name: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    Forecast {
        model_name: String,
        #[arg(default_value = "7")]
        days: usize,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(Subcommand)]
enum TrainCommand {}

#[derive(Subcommand)]
enum ChecksumCommand {
    Fingerprint,
    Verify,
}

/// Dispatch models to be evaluated and return results
pub struct Dispatch {
    cli: Cli,
}

impl Dispatch {
    pub fn new() -> Self {
        let cli = Cli::parse();

        Dispatch { cli }
    }

    pub async fn dispatch(&self) -> Result<(), DispatchError> {
        match &self.cli.command {
            // data procedures
            Commands::Init => initialize().await,

            Commands::Sync => update_local_files().await,

            Commands::Checksums { action } => match action {
                ChecksumCommand::Verify => {
                    let expected =
                        ChecksumMap::load().map_err(|e| DispatchError::ChecksumLoadError(e))?;
                    let actual = generate_checksums();

                    if expected != actual {
                        let mismatched_era_paths = expected.diff(&actual);
                        let mut f_str = String::new();

                        for path in mismatched_era_paths {
                            f_str.push_str(&format!("\n📄 {}", path.display()));
                        }
                        println!("❌ checksums do not match for eras:{f_str}");
                    } else {
                        println!("✅ checksums match serialized checksum map. data is intact.");
                    }

                    Ok(())
                }

                ChecksumCommand::Fingerprint => {
                    let checksums = generate_checksums();

                    checksums
                        .save()
                        .map_err(|_| DispatchError::ChecksumSerializationError)?;

                    Ok(())
                }
            },
            // model prodecures
            Commands::Train { model_name, args } => {
                let mut model = get_model_from_inventory(model_name, args)?;

                let start = Instant::now();

                model.train(Chronology::new());

                println!(
                    "✅ successfully trained {} in {}ms",
                    model.model_name(),
                    start.elapsed().as_millis()
                );
                Ok(())
            }

            Commands::Forecast {
                model_name,
                days,
                args,
            } => {
                let mut model = get_model_from_inventory(model_name, &args)?;

                model
                    .initialize()
                    .map_err(|_| DispatchError::ModelNotTrained(model_name.to_owned()))?;

                let predictions = forecast_nba(model, *days)
                    .await
                    .map_err(|e| DispatchError::ForecastError(e))?;

                println!("{}", format::bar(80));
                for prediction in predictions {
                    println!("{prediction}");
                }

                Ok(())
            }

            Commands::Evaluate { model_name, args } => {
                let mut model = get_model_from_inventory(model_name, &args)?;

                model
                    .initialize()
                    .map_err(|_| DispatchError::ModelNotTrained(model_name.to_owned()))?;

                let results = model.evaluate();

                println!(
                    "{}",
                    format::evaluation(model.model_name().to_ascii_uppercase(), &results)
                );

                Ok(())
            }
        }
    }
}

fn get_model_from_inventory(
    model_name: &str,
    args: &[String],
) -> Result<Box<dyn Model>, DispatchError> {
    let registration = inventory::iter::<Registration>()
        .find(|r| r.model_name == model_name)
        .ok_or_else(|| DispatchError::UnknownModel(model_name.to_owned()))?;

    let mut full_args = vec![model_name.to_string()];
    full_args.extend_from_slice(args);

    let matches = (registration.args_schema)()
        .try_get_matches_from(full_args.iter())
        .map_err(|e| DispatchError::ArgumentParseError(e))?;

    let model = (registration.factory)(&matches);

    Ok(model)
}

#[derive(Debug, Error)]
pub enum DispatchError {
    #[error("❌ source data was not correctly serialized. ")]
    SourceDataError,
    #[error("{0}\n❌ nba files in storage are malformed: training data could not be interpreted")]
    HistoryError(ChronologyError),
    #[error("❌ failed to initialize NBA data. ")]
    InitializationError,
    #[error("{0}\n❌ failed to load checksums from file.")]
    ChecksumLoadError(ChecksumMapError),
    #[error("❌ failed to serialize checksums to file: {}", nba_checksum_file().display())]
    ChecksumSerializationError,
    #[error("{0}\n❌ failed to create predictions for upcoming NBA games. ")]
    ForecastError(ForecastError),
    #[error("❌ model {0} is not trained. try running `warheads train {0}` ")]
    ModelNotTrained(String),
    #[error("❌ unknown model '{0}'. ")]
    UnknownModel(String),
    #[error("{0}\n❌ failed to parse command line arguments.")]
    ArgumentParseError(clap::error::Error),
}

async fn initialize() -> Result<(), DispatchError> {
    observe_nba().await;
    annotate_nba().await;
    chronicle_nba();
    println!("✅ successfully initialized NBA data in warheads directory.");
    Ok(())
}

async fn update_local_files() -> Result<(), DispatchError> {
    let _ = update_source_data()
        .await
        .map_err(|_| DispatchError::SourceDataError)?;
    println!("✅ successfully updated source data.");

    let _ = chronicle_nba();
    println!("✅ NBA volumes created successfully.");

    Ok(())
}
