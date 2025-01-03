// Struct
use crate::logger::Logger;
// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static LOGGER: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| Arc::new(Mutex::new(Logger::default())));
