use crate::logging::{LogMessage, LogSeverity};
use std::{
    sync::{atomic::AtomicU64, mpsc::Sender},
    thread::JoinHandle,
};

mod drop;
mod frame;
mod log;
mod logger;
mod new;
mod should_log;

/// The central controller for all logging in the engine
pub struct LogController {
    /// The minimum severity to log
    minimum_severity: LogSeverity,

    /// The number of frames that have been presented since startup
    frame: AtomicU64,

    /// The reported startup time of the program in CPU ticks
    start_ticks: u64,

    /// The frequency the peformance counter runs at, in CPU ticks/second
    performance_counter_frequency: u64,

    /// The queue for sending log messages to the logger thread
    message_queue: Sender<Option<LogMessage>>,

    /// The handle to the logging thread
    join_handle: Option<JoinHandle<()>>,
}
