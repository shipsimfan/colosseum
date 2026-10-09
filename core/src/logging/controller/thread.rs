use crate::{
    LogController, Result,
    logging::{LogMessage, LogOutput},
};
use std::sync::mpsc::Receiver;

impl LogController {
    /// The main function for the logging thread
    pub(in crate::logging) fn log_thread(
        messages: Receiver<Option<LogMessage>>,
        mut outputs: Vec<Box<dyn LogOutput>>,
    ) -> Result<()> {
        while let Ok(Some(message)) = messages.recv() {
            for output in &mut outputs {
                output.output(&message)?;
            }
        }

        Ok(())
    }
}
