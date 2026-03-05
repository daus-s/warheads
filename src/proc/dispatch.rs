use std::env;
use thiserror::Error;
use DispatchError::*;

use crate::checksum::checksum_map::{ChecksumMap, ChecksumMapError};
use crate::checksum::generate::generate_checksums;

use crate::format::path_manager::nba_checksum_file;
use crate::proc::forecast::forecast_nba;
use crate::proc::historian::{annotate_nba, chronicle_nba, observe_nba};
use crate::proc::refresher::update_source_data;

/// dispatch models to be evalutated and return results
pub struct Dispatch {
    args: Vec<String>,
}

impl Dispatch {
    pub fn new() -> Self {
        let args = env::args().collect();

        Dispatch { args }
    }

    pub async fn dispatch(&self) -> Result<(), DispatchError> {
        if self.args.len() < 2 {
            println!("{}", USAGE);
            return Ok(());
        }

        match self.args[1].as_str() {
            "init" => initialize().await,
            "sync" => match update_source_data().await {
                Ok(_) => {
                    println!("✅ successfully updated source data. ");
                    Ok(())
                }
                Err(_) => {
                    println!("❌ failed to update source data. ");
                    Err(DispatchError::SourceDataError)
                }
            },
            "train" => {
                assert!(self.args.len() > 2, "todo: write model training usage");
                //replace with trainable trait include in model?
                match self.args[1].as_str() {
                    "elo-tracker" => {
                        todo!("add options")
                    }
                    _ => {
                        todo!("model usage")
                    }
                }
            }
            "help" => {
                println!("{}", USAGE);
                Ok(())
            }
            "checksums" => {
                assert!(self.args.len() > 2, "todo: write checksums usage");

                match self.args[2].as_str() {
                    "verify" => {
                        // let actual = generate_checksums();
                        let expected =
                            ChecksumMap::load().map_err(|e| DispatchError::ChecksumLoadError(e))?;
                        let actual = generate_checksums();

                        if expected != actual {
                            let mismatched_eras = expected.diff(&actual);

                            let mut f_str = String::new();

                            for era in mismatched_eras {
                                f_str.push_str(&format!("\n📄 {}", era.display()));
                            }
                            println!("❌ checksums do not match for eras:{f_str}",);
                        } else {
                            println!("✅ checksums match serialized checksum map. data is intact. ")
                        }

                        Ok(())
                    }
                    "generate" => {
                        //todo: generate checksums
                        //
                        let checksums = generate_checksums();

                        checksums
                            .save()
                            .map_err(|_| DispatchError::ChecksumSerializationError)?;

                        Ok(())
                    }
                    _ => {
                        todo!("checksums usage")
                    }
                }
            }
            "forecast" => {
                let ratings = load_historical_ratings();

                forecast_nba(elo).await;

                Ok(())
            }
            "tweet" => {
                todo!("implement tweet")
            }
            x => Err(UnrecognizedCommand(x.to_owned())),
        }
    }
}

const USAGE: &'static str = r#"                                    warheads API
this program provides a frameowrk to generate and train ML models on nba data.
it requires network access and will make minimal requests to the
===============================================================================
USAGE

DATA
    init - initializes

MODELS
the model trait and API is exposed to the user. see the model module for more
documentation.

to run and train. if data cannot be loaded these procedures may tell you to run
other routines to update data.
===============================================================================
LICENSE
===============================================================================

"#;

#[derive(Debug, Error)]
pub enum DispatchError {
    #[error("❌ the argument {0} is not a known command. for known commands run warheads help")]
    UnrecognizedCommand(String),
    #[error("❌ source data was not correctly serialized. ")]
    SourceDataError,
    #[error("❌ failed to initialize NBA data. ")]
    InitializationError,
    #[error("{0}\n❌ failed to load checksums from file.")]
    ChecksumLoadError(ChecksumMapError),
    #[error("❌ failed to serialize checksums to file: {}", nba_checksum_file().display())]
    ChecksumSerializationError,
}

async fn initialize() -> Result<(), DispatchError> {
    observe_nba().await;

    annotate_nba().await;

    chronicle_nba();

    println!("✅ successfully initialized NBA data in warheads directory. ");
    Ok(())
}
