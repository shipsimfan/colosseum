use crate::{Error, ThreadManager, file_io::FileIo};
use std::sync::mpsc::channel;

impl FileIo {
    /// Create a new [`FileIo`] thread`
    pub(crate) fn new(thread_manager: &ThreadManager) -> Result<FileIo, Error> {
        // Create the channel for sending file I/O requests to the thread
        let (sender, receiver) = channel();

        // Spawn the file I/O thread
        let child_sender = sender.clone();
        thread_manager.spawn(
            "File I/O".to_string(),
            move |shared_state| {
                FileIo::thread(shared_state.logger().logger("file"), receiver);
                Ok(())
            },
            move || {
                child_sender.send(None).ok();
            },
        )?;

        Ok(FileIo { sender })
    }
}
