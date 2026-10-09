use crate::{
    LogSeverity,
    logging::{LogMessage, LogOutput},
};
use std::{
    sync::{
        Mutex,
        atomic::AtomicU64,
        mpsc::{Receiver, Sender},
    },
    time::Instant,
};

mod frame;
mod log;
mod logger;
mod new;
mod should_log;
mod spawn_thread;
mod r#static;
mod thread;

/// The central controller for all logging in the engine
pub struct LogController {
    /// The minimum severity to log
    minimum_severity: LogSeverity,

    /// The number of frames that have been presented since startup
    frame: AtomicU64,

    /// The reported startup time of the program
    start_time: Instant,

    /// The queue for sending log messages to the logger thread
    message_queue: Sender<Option<LogMessage>>,

    /// The outputs and receiver to pass to the main logging thread
    outputs: Mutex<Option<(Vec<Box<dyn LogOutput>>, Receiver<Option<LogMessage>>)>>,
}
