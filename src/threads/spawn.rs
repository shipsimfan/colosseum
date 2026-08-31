use crate::{GlobalSharedState, Result, ThreadManager, threads::Thread};

impl ThreadManager {
    /// Spawn a new thread
    pub fn spawn<
        F1: 'static + FnOnce(&GlobalSharedState) -> Result<()> + Send,
        F2: 'static + FnOnce() + Send,
    >(
        &self,
        name: String,
        f: F1,
        on_kill: F2,
    ) -> Result<()> {
        if !self.shared_state.is_running() {
            return Ok(());
        }

        let thread = Thread::new(name, self.shared_state.clone(), f, on_kill, &self.logger)?;
        self.threads.lock().unwrap().push(thread);
        Ok(())
    }
}
