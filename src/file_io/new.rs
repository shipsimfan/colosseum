use crate::{Error, FileIo, UserEvent};
use colosseum_core::{ThreadManager, logger};
use std::sync::mpsc::channel;

impl FileIo {
    /// Create a new [`FileIo`] thread`
    pub(crate) fn new(thread_manager: &ThreadManager<UserEvent>) -> Result<FileIo, Error> {
        // Create the channel for sending file I/O requests to the thread
        let (sender, receiver) = channel();

        // Spawn the file I/O thread
        let child_sender = sender.clone();
        thread_manager.spawn(
            "File I/O".to_string(),
            move |_| {
                FileIo::thread(logger!("file"), receiver);
                Ok(())
            },
            move || {
                child_sender.send(None).ok();
            },
        )?;

        Ok(FileIo { sender })
    }
}
