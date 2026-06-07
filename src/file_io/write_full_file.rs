use crate::file_io::{FileIo, WriteFullFile, WriteFullFileOp};
use std::path::PathBuf;

impl FileIo {
    /// Write a full file to disk, creating it if it doesn't exist and overwriting it if it does
    pub fn write_full_file<P: Into<PathBuf>, D: Into<Vec<u8>>>(
        &self,
        path: P,
        data: D,
    ) -> WriteFullFile {
        let (operation, handle) = WriteFullFileOp::new(path.into(), data.into());
        self.sender.send(Some(operation.into())).ok();
        handle
    }
}
