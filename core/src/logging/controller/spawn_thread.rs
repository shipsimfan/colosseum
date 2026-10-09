use crate::{LogController, Result, ThreadManager};

impl LogController {
    /// Spawn the logger thread
    pub(crate) fn spawn_thread<UserData: 'static + Send>(
        &self,
        thread_manager: &ThreadManager<UserData>,
    ) -> Result<()> {
        let (outputs, receiver) = self
            .outputs
            .lock()
            .unwrap()
            .take()
            .expect("attempting to start the logging thread more than once");

        let message_queue = self.message_queue.clone();
        thread_manager.spawn(
            "Logging".to_string(),
            move |_| LogController::log_thread(receiver, outputs),
            move || {
                message_queue.send(None).ok();
            },
        )?;

        Ok(())
    }
}
