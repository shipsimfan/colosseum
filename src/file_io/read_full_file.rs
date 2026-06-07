use crate::{
    Result,
    file_io::{FileIo, ReadFullFile, ReadFullFileOp},
};
use std::path::PathBuf;

impl FileIo {
    /// Read the entire contents of a file at the given path
    pub fn read_full_file<P: Into<PathBuf>>(&self, path: P) -> ReadFullFile {
        let (operation, handle) = ReadFullFileOp::new(path.into(), false).unwrap();
        self.sender.send(Some(operation.into())).ok();
        handle
    }

    /// Read the entire contents of a file at the given path, blocking until the read is complete
    pub(crate) fn read_full_file_blocking<P: Into<PathBuf>>(&self, path: P) -> Result<Vec<u8>> {
        let (operation, handle) = ReadFullFileOp::new(path.into(), true)?;
        self.sender.send(Some(operation.into())).ok();
        handle.wait()
    }
}
