use crate::{
    RunningState,
    logging::{LogMessage, LogOutput},
};
use std::sync::{Arc, mpsc::Receiver};

/// The main function for the logging thread
pub(in crate::logging) fn log_thread(
    messages: Receiver<Option<LogMessage>>,
    mut outputs: Vec<Box<dyn LogOutput>>,
    running_state: Arc<RunningState>,
) {
    while let Some(message) = messages.recv().unwrap() {
        for output in &mut outputs {
            output.output(&message);
        }
    }

    running_state.kill();
}
