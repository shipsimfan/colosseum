use crate::logging::{LogMessage, LogSeverity};
use std::{
    sync::{atomic::AtomicU64, mpsc::Sender},
    time::Instant,
};

mod start_token;

mod frame;
mod log;
mod logger;
mod new;
mod should_log;
mod spawn_thread;

pub(crate) use start_token::LogStartToken;

/// The central controller for all logging in the engine
pub struct LogController {
    /// The minimum severity to log
    minimum_severity: LogSeverity,

    /// The number of frames that have been presented since startup
    frame: AtomicU64,

    /// The reported startup time of the program
    start_time: Instant,

    /// The queue for sending log messages to the logger thread
    message_queue: Sender<LogMessage>,
}
