// Struct
use crate::services::{Action, FgBg, Service};
use crate::types::{Attach, Commands, DetachableCommands, PostCommands};
use pipelight_exec::Status;
use workflow::{Config, Getters, Node, Pipeline};
// Globals
use crate::globals::CLI;
use crate::types::verbosity::{level_value, Verbosity};
use log::LevelFilter;
use pipelight_utils::globals::LOGGER;

// Error Handling
use super::EXIT_CODE;
use miette::{Error, Result};
use std::process::{ExitCode, Termination};

pub fn launch() -> Result<()> {
    let mut args = CLI.lock().unwrap().clone();

    // Retrieve command line args
    let name: String;
    match args.commands.clone() {
        Commands::PostCommands(PostCommands::DetachableCommands(DetachableCommands::Run(e))) => {
            name = e.name.unwrap();
        }
        _ => {
            let message = "Couldn't retrieve pipeline name";
            return Err(Error::msg(message));
        }
    };

    let mut pipeline = Pipeline::get_by_name(&name)?;
    let config = Config::get()?;

    // Guard
    pipeline.is_triggerable()?;
    if args.verbose.log_level_filter() == LevelFilter::Error {
        // Retrieve global options
        if config.has_loglevel_option().unwrap() {
            if let Some(level_filter) = config.get_default_loglevel().ok() {
                let level = level_filter.to_level();
                args.verbose = Verbosity::new(level_value(level).try_into().unwrap(), 0);
                LOGGER.lock().unwrap().set_level(&level_filter)?;
            }
        }
        // Retrieve per-pipeline options
        if pipeline.has_loglevel_option().unwrap() {
            if let Some(level_filter) = pipeline.get_default_loglevel().ok() {
                let level = level_filter.to_level();
                args.verbose = Verbosity::new(level_value(level).try_into().unwrap(), 0);
                LOGGER.lock().unwrap().set_level(&level_filter)?;
            }
        }
    }

    if args.attach.is_none() {
        // Retrieve global options
        if config.has_attach_option().unwrap() {
            args.attach = Some(config.options.unwrap().attach.unwrap().to_string());
            // args.attach = Some(!config.should_detach().unwrap());
        }
        // Retrieve per-pipeline options
        if pipeline.has_attach_option().unwrap() {
            args.attach = Some(
                pipeline
                    .clone()
                    .options
                    .unwrap()
                    .attach
                    .unwrap()
                    .to_string(),
            );
        }
    }

    // Action
    match args.attach.clone() {
        Some(val) => {
            if val == String::from(&Attach::True) {
                pipeline.run()?;
                // Return pipeline log
                println!("{}", Node::from(&pipeline));

                let _ = match pipeline.status {
                    Some(Status::Succeeded) => {
                        *EXIT_CODE.lock().unwrap() = ExitCode::SUCCESS;
                        Ok(())
                    }
                    Some(Status::Failed) => {
                        *EXIT_CODE.lock().unwrap() = ExitCode::FAILURE;
                        let message = "Pipeline status: Failed";
                        Err(Error::msg(message))
                    }
                    Some(Status::Aborted) => {
                        *EXIT_CODE.lock().unwrap() = ExitCode::FAILURE;
                        let message = "Pipeline status: Aborted";
                        Err(Error::msg(message))
                    }
                    _ => Ok(()),
                };
            }
            if val == String::from(&Attach::False) {
                Service::new(Action::RunLoose, Some(args))?.should_detach()?;
            }
        }
        None => {
            Service::new(Action::RunLoose, Some(args))?.should_detach()?;
        }
    };
    Ok(())
}
