use crate::file_io::ReadFullFile;

impl ReadFullFile {
    /// Has the read completed?
    pub fn is_complete(&self) -> bool {
        self.result.is_available()
    }
}
