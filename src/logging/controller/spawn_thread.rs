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
        thread_manager: &ThreadManager,
    ) -> Result<()> {
        let message_queue = self.message_queue.clone();
        thread_manager.spawn(
            "Logging".to_string(),
            move |_| log_thread(start_token.receiver, start_token.outputs),
            move || {
                message_queue.send(None).ok();
            },
        )
    }
}

/// The main function for the logging thread
fn log_thread(
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
