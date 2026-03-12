use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::checksum::checksum_map::{ChecksumMap, ChecksumMapError};
use crate::checksum::generate::generate_checksums;

use crate::format::path_manager::nba_checksum_file;
use crate::ml::elo_tracker::EloTracker;
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
        //optional params
    },
    #[command(name = "eval")]
    Evaluate {
        model_name: String,
    },
    Forecast {
        model_name: String,
        #[arg(default_value = "7")]
        days: usize,
    },
    Tweet,
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

            Commands::Sync => match update_source_data().await {
                Ok(_) => {
                    println!("✅ successfully updated source data.");
                    Ok(())
                }
                Err(_) => {
                    println!("❌ failed to update source data.");
                    Err(DispatchError::SourceDataError)
                }
            },

            Commands::Checksums { action } => match action {
                ChecksumCommand::Verify => {
                    let expected =
                        ChecksumMap::load().map_err(|e| DispatchError::ChecksumLoadError(e))?;
                    let actual = generate_checksums();

                    if expected != actual {
                        let mismatched_eras = expected.diff(&actual);
                        let mut f_str = String::new();

                        for era in mismatched_eras {
                            f_str.push_str(&format!("\n📄 {}", era.display()));
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
            Commands::Train { model_name } => {
                todo!()
            }

            Commands::Forecast { model_name, days } => {
                let mut model = EloTracker::new();

                let data = Chronology::new()
                    .as_training_data()
                    .map_err(|e| DispatchError::HistoryError(e))?;

                model.train(&data);

                let predictions = forecast_nba(model, *days)
                    .await
                    .map_err(|e| DispatchError::ForecastError(e))?;

                for prediction in predictions {
                    println!("{prediction:?}");
                }

                Ok(())
            }

            Commands::Evaluate { model_name } => {
                let model_factory = inventory::iter::<Registration>()
                    .find(|r| r.model_name == model_name)
                    .ok_or_else(|| DispatchError::UnknownModel(model_name.clone()))?
                    .factory;

                let model = model_factory();

                let results = model.evaluate();

                println!("{results:?}");

                Ok(())
            }

            Commands::Tweet => {
                todo!("implement tweet")
            }
        }
    }
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
    #[error("❌ unknown model '{0}'. ")]
    UnknownModel(String),
}

async fn initialize() -> Result<(), DispatchError> {
    observe_nba().await;
    annotate_nba().await;
    chronicle_nba();
    println!("✅ successfully initialized NBA data in warheads directory.");
    Ok(())
}
