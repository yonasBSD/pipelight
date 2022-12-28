#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::LevelFilter::{Debug, Trace};
#[allow(dead_code)]
use shared::config::get_pipeline;
use shared::logger::{debug, error, info, set_logger, trace, warn};
use shared::types::logs::{PipelineLog, PipelineStatus, StepLog};
use shared::types::{Pipeline, Step};
use std::env;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|e| {
        // println!("{}", e);
        exit(1)
    })
}

/// Launch attached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    let handle = set_logger(Trace)?;
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();
    let p: Pipeline = Config::get()?.pipelines.get(&pipeline_name)?;
    let mut pipeline = PipelineLog::from(p);
    pipeline.run(handle);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::type_of;
    #[test]
    fn internal() {
        let res = load_config();
    }
}