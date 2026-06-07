use crate::file_io::{FileIoOperation, ReadFullFileOp, WriteFullFileOp};

impl From<ReadFullFileOp> for FileIoOperation {
    fn from(op: ReadFullFileOp) -> Self {
        FileIoOperation::ReadFullFile(op)
    }
}

impl From<WriteFullFileOp> for FileIoOperation {
    fn from(op: WriteFullFileOp) -> Self {
        FileIoOperation::WriteFullFile(op)
    }
}
