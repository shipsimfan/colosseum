use crate::{Result, file_io::WriteFullFile};

impl WriteFullFile {
    /// Get the result of the write operation
    ///
    /// # Panics
    /// Panics if the write operation has not completed yet. Use [`WriteFullFile::is_complete`] to
    /// check if the operation has completed before calling this method
    pub fn result(self) -> Result<()> {
        self.result.take()
    }
}
