use crate::{Result, threads::Thread};

impl Thread {
    /// Join the thread, returning any error that occurred in the thread
    pub fn join(self) -> Result<()> {
        (self.on_kill)();
        self.join_handle.join().unwrap();
        self.result.take()
    }
}
