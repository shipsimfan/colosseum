use crate::ReadMeshFile;

impl ReadMeshFile {
    /// Has the read completed?
    pub fn is_complete(&self) -> bool {
        self.result.is_available()
    }
}
