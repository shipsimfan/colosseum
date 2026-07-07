use crate::{Result, error, logging::Logger, threads::Thread};

impl Thread {
    /// Join the thread, returning any error that occurred in the thread
    pub fn join(self, logger: &Logger) -> Result<()> {
        (self.on_kill)();
        if self.join_handle.join().is_err() {
            error!(logger, "Thread \"{}\" panicked", self.name);
        }
        let result = self.result.take();
        if let Err(error) = &result {
            error!(
                logger,
                "Thread \"{}\" ended with error: {}", self.name, error
            );
        }
        result
    }
}
