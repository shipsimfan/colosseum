use crate::WriteFullFile;

impl WriteFullFile {
    /// Has the write completed?
    pub fn is_complete(&self) -> bool {
        self.result.is_available()
    }
}
