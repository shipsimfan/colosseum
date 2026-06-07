use crate::{file_io::FileIoOperation, logging::Logger};

impl FileIoOperation {
    /// Execute the file I/O operation
    pub fn execute(self, logger: &Logger) {
        match self {
            FileIoOperation::ReadFullFile(op) => op.execute(logger),
            FileIoOperation::WriteFullFile(op) => op.execute(logger),
        }
    }
}
