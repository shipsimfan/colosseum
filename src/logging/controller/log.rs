use crate::logging::{LogController, LogMessage, LogSeverity};
use std::sync::atomic::Ordering;
use win32::{LARGE_INTEGER, QueryPerformanceCounter};

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
        // Get the milliseconds
        let mut ticks = LARGE_INTEGER::default();
        unsafe { QueryPerformanceCounter(&mut ticks) };
        let ticks = unsafe { ticks.quad_part } as u64 - self.start_ticks;

        let q = ticks / self.performance_counter_frequency;
        let r = ticks % self.performance_counter_frequency;

        let milliseconds = q * 1000 + (r * 1000) / self.performance_counter_frequency;

        // Get the frame
        let frame = self.frame.load(Ordering::Acquire);

        // Send the message to the thread
        let message = LogMessage::new(severity, message, frame, milliseconds, scope, module);
        self.message_queue.send(Some(message)).ok();
    }
}
