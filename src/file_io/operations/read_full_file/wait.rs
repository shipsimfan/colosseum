use crate::{Result, file_io::ReadFullFile};

impl ReadFullFile {
    /// Wait for the file read operation to complete and return the file contents
    pub fn wait(self) -> Result<Vec<u8>> {
        self.result.wait().flatten()
    }
}
