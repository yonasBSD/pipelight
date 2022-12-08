#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
use log::{debug, error, info, trace, warn};
use shared::actions;
use shared::cli;
use shared::logger;
use shared::shell;
use shared::types;

fn main() {
    shell::load_config();
    logger::set_logger_config();
    cli::get_args();
}