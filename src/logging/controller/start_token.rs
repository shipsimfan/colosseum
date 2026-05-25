use crate::logging::{LogMessage, LogOutput};
use std::sync::mpsc::Receiver;

/// An opaque token to start the logging thread after the thread controller has been made
pub(crate) struct LogStartToken {
    /// The receiver for the logging thread to recieve messages on
    pub(in crate::logging::controller) receiver: Receiver<LogMessage>,

    /// The outputs for the logging thread to use
    pub(in crate::logging::controller) outputs: Vec<Box<dyn LogOutput>>,
}
