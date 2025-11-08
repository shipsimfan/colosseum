use crate::logging::LogController;

impl Drop for LogController {
    fn drop(&mut self) {
        // Send quit command
        self.message_queue.send(None).ok();

        // Wait for the logger thread to finish
        self.join_handle.take().unwrap().join().unwrap();
    }
}
