use alexandria::Severity;
use command::Command;
use std::{
    borrow::Cow,
    fmt::{Debug, Formatter},
    path::PathBuf,
    sync::mpsc::Sender,
    thread::JoinHandle,
    time::SystemTime,
};

mod command;
mod thread;

/// A thread which writes messages to files
#[derive(Clone)]
pub(super) struct FileWriter {
    /// The sender to the log writer thread
    sender: Sender<Command>,
}

impl FileWriter {
    /// Spawns a new [`FileWriter`] thread
    pub(super) fn new(directory: PathBuf) -> (FileWriter, JoinHandle<()>) {
        let (sender, queue) = std::sync::mpsc::channel();

        let join_handle = std::thread::spawn(move || thread::run(queue, directory));

        (FileWriter { sender }, join_handle)
    }

    /// Opens the file for `name`
    pub(super) fn open(&self, name: &'static str) {
        self.sender.send(Command::Open(name)).ok();
    }

    /// Logs `message` for `name`
    pub(super) fn log(&self, severity: Severity, name: &'static str, message: Cow<'static, str>) {
        self.sender
            .send(Command::Write(name, severity, message, SystemTime::now()))
            .ok();
    }

    /// Closes the file for `name`
    pub(super) fn close(&self, name: &'static str) {
        self.sender.send(Command::Close(name)).ok();
    }
}

impl Debug for FileWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("FileWriter")
    }
}
