use clap::{Parser, Subcommand};
use thiserror::Error;

use crate::checksum::checksum_map::{ChecksumMap, ChecksumMapError};
use crate::checksum::generate::generate_checksums;

use crate::format::path_manager::nba_checksum_file;
use crate::ml::elo_tracker::EloTracker;
use crate::ml::model::Model;
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
        #[command(subcommand)]
        model: TrainCommand,
    },
    Evaluate {
        #[command(subcommand)]
        model: EvaluateCommand,
    },
    Forecast {
        #[arg(default_value = "7")]
        days: usize,
    },
    Tweet,
}

#[derive(Subcommand)]
enum TrainCommand {
    /// Train ELO tracker model
    Elo {
        /// Name of the model
        #[arg(long)]
        model_name: String,

        /// Learning rate
        #[arg(long)]
        lr: Option<f32>,

        /// Number of epochs
        #[arg(long)]
        epochs: Option<u32>,
    },
    // Add more model types here as needed
}

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

            Commands::Train { model } => {
                todo!()
            }

            Commands::Forecast { days } => {
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
}

async fn initialize() -> Result<(), DispatchError> {
    observe_nba().await;
    annotate_nba().await;
    chronicle_nba();
    println!("✅ successfully initialized NBA data in warheads directory.");
    Ok(())
}
