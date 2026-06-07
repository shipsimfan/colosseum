//! Module for performing asynchronous file I/O operations

use std::sync::mpsc::Sender;

mod operations;

mod new;
mod read_full_file;
mod thread;
mod write_full_file;

pub use operations::*;

/// Access for performing asynchronous file I/O operations
#[derive(Clone)]
pub struct FileIo {
    /// The sender for sending file I/O operations to the file I/O thread
    sender: Sender<Option<FileIoOperation>>,
}
