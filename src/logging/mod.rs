//! Logging utilities for the game engine

use formatters::*;
use message::LogMessage;
use options::{FormatterKind, LogPath};
use outputs::*;
use thread::log_thread;

mod controller;
mod formatters;
mod logger;
mod message;
mod options;
mod outputs;
mod thread;

pub use controller::LogController;
pub use logger::Logger;
pub use message::LogSeverity;

pub(crate) use options::LoggingOptions;
