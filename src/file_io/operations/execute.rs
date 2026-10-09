use crate::{Logger, file_io::FileIoOperation};

impl FileIoOperation {
    /// Execute the file I/O operation
    pub fn execute(self, logger: &Logger) {
        match self {
            FileIoOperation::ReadFullFile(op) => op.execute(logger),
            FileIoOperation::WriteFullFile(op) => op.execute(logger),
        }
    }
}
