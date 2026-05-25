use std::sync::mpsc::Receiver;

use crate::{
    Result, ThreadManager,
    logging::{LogController, LogMessage, LogOutput, LogStartToken},
};

impl LogController {
    /// Spawn the logger thread
    pub(crate) fn spawn_thread(
        &self,
        start_token: LogStartToken,
        thread_manager: &mut ThreadManager,
    ) -> Result<()> {
        thread_manager.spawn("Logging".to_string(), move |_| {
            log_thread(start_token.receiver, start_token.outputs)
        })
    }
}

/// The main function for the logging thread
fn log_thread(messages: Receiver<LogMessage>, mut outputs: Vec<Box<dyn LogOutput>>) -> Result<()> {
    while let Ok(message) = messages.try_recv() {
        for output in &mut outputs {
            output.output(&message)?;
        }
    }

    Ok(())
}
