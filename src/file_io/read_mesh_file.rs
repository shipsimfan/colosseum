use crate::{FileIo, ReadMeshFile, file_io::ReadMeshFileOp};
use std::path::PathBuf;

impl FileIo {
    /// Read a mesh file asynchronously
    pub(crate) fn read_mesh_file(&self, path: PathBuf) -> ReadMeshFile {
        let (operation, handle) = ReadMeshFileOp::new(path, false).unwrap();
        self.sender.send(Some(operation.into())).ok();
        handle
    }
}
