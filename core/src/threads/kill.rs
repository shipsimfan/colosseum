use crate::{Error, ThreadManager, new_error};

impl<UserData: 'static + Send> ThreadManager<UserData> {
    /// Signal all threads to stop running and join them
    pub fn kill(&self, source: &str) -> Result<(), Vec<Error>> {
        self.shared_state.kill(source);

        let mut errors = Vec::new();

        if let Ok(Some(receiver)) = self
            .panic_receiver
            .try_lock()
            .map(|mut receiver| receiver.take())
        {
            if let Ok(panic) = receiver.try_take() {
                errors.push(new_error!(panic));
            }
        }

        let mut threads = self.threads.lock().unwrap();
        for thread in threads.drain(..).rev() {
            if let Err(error) = thread.join(self.shared_state.logger()) {
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
