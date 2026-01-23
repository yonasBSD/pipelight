// Filesystem - read files
use pipelight_utils::file::read_last_line;
use std::fs;
use std::path::Path;
// Traits
use serde::{Deserialize, Serialize};
// Error Handling
use log::{trace, warn};
use miette::{Error, IntoDiagnostic, Result};
use pipelight_error::{LibError, PipelightError};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Logs;

/**
* Provide a directory path of valid JSON5 pipelight logs.
* It returns a list/vec of valid json strings
* further to be converted into Pipeline structs in the core("workflow") crate.
*/
impl Logs {
    pub fn read(directory_path: &str) -> Result<Vec<String>, PipelightError> {
        let mut logs: Vec<String> = vec![];
        // Directory Safe-guard
        let message = format!("Reading log directory: {}", directory_path);
        trace!("{}", message);
        if !Path::new(directory_path).exists() {
            let message = "No logs to display.";
            let err = LibError::builder().msg(message).help("").build();
            return Err(err.into());
        }
        // Files Safe-guard
        let entries = fs::read_dir(directory_path)?;
        for entry in entries {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let res = read_last_line(&entry.path());
                match res {
                    Ok(json) => {
                        logs.push(json);
                    }
                    Err(_err) => {
                        warn!("Stripped corrupted log file: {}", entry.path().display())
                    }
                }
            }
        }
        Ok(logs)
    }
}
