use crate::{GlobalSharedState, Result, ThreadManager, threads::Thread};

impl ThreadManager {
    /// Spawn a new thread
    pub fn spawn<F: 'static + FnOnce(&GlobalSharedState) -> Result<()> + Send>(
        &mut self,
        name: String,
        f: F,
    ) -> Result<()> {
        let thread = Thread::new(name, f, self.shared_state.clone())?;
        self.threads.push(thread);
        Ok(())
    }
}
