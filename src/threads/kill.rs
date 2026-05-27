use crate::{Error, ThreadManager};

impl ThreadManager {
    /// Signal all threads to stop running and join them
    pub fn kill(&self) -> Result<(), Vec<Error>> {
        self.shared_state.kill();

        let mut errors = Vec::new();

        let mut threads = self.threads.lock().unwrap();
        for thread in threads.drain(..).rev() {
            if let Err(error) = thread.join() {
                errors.push(error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
