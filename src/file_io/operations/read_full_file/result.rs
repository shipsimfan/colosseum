use crate::{Result, file_io::ReadFullFile};

impl ReadFullFile {
    /// Get the result of the read operation
    ///
    /// # Panics
    /// Panics if the read operation has not completed yet. Use [`ReadFullFile::is_complete`] to
    /// check if the operation has completed before calling this method
    pub fn result(self) -> Result<Vec<u8>> {
        self.result.take()
    }
}
