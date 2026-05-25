use crate::{Result, threads::Thread};

impl Thread {
    /// Join the thread, returning any error that occurred in the thread
    pub fn join(self) -> Result<()> {
        self.join_handle.join().unwrap();

        if let Some(result) = self.shared_state.get_result() {
            return Box::into_inner(result);
        }

        Ok(())
    }
}
