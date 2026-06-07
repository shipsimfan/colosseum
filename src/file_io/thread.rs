use crate::{
    file_io::{FileIo, FileIoOperation},
    logging::Logger,
};
use std::sync::mpsc::Receiver;

impl FileIo {
    /// The file I/O thread function
    pub(in crate::file_io) fn thread(logger: Logger, receiver: Receiver<Option<FileIoOperation>>) {
        while let Ok(Some(operation)) = receiver.recv() {
            operation.execute(&logger);
        }
    }
}
