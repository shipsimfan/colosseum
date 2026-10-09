use crate::{FileIo, Logger, file_io::FileIoOperation};
use std::sync::mpsc::Receiver;

impl FileIo {
    /// The file I/O thread function
    pub(in crate::file_io) fn thread(logger: Logger, receiver: Receiver<Option<FileIoOperation>>) {
        while let Ok(Some(operation)) = receiver.recv() {
            operation.execute(&logger);
        }
    }
}
