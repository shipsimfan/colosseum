use crate::logging::{LogController, LogMessage, LogSeverity};
use std::sync::atomic::Ordering;

impl LogController {
    /// Log a message
    ///
    /// This function does not check if the message should be emitted, use
    /// [`LogController::should_log`] first
    pub(in crate::logging) fn log(
        &self,
        severity: LogSeverity,
        message: String,
        scope: &'static str,
        module: &'static str,
    ) {
        // Get the current time in milliseconds since startup
        let milliseconds = self.start_time.elapsed().as_millis() as u64;

        // Get the frame
        let frame = self.frame.load(Ordering::Acquire);

        // Send the message to the thread
        let message = LogMessage::new(severity, message, frame, milliseconds, scope, module);
        self.message_queue.send(message).ok();
    }
}
