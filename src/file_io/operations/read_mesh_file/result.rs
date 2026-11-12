use crate::{ReadMeshFile, Result};

impl ReadMeshFile {
    /// Get the result of the read operation
    ///
    /// # Panics
    /// Panics if the read operation has not completed yet. Use [`ReadMeshFile::is_complete`] to
    /// check if the operation has completed before calling this method
    pub fn result(self) -> Result<parthenon::Mesh> {
        self.result.take()
    }
}
