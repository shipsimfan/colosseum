mod read_full_file;
mod write_full_file;

mod execute;
mod from;

pub use read_full_file::*;
pub use write_full_file::*;

/// Operations that the file I/O thread can perform
pub(in crate::file_io) enum FileIoOperation {
    /// Read the entire contents of a file into memory
    ReadFullFile(ReadFullFileOp),

    /// Write the entire contents of a file from memory, creating the file if it doesn't exist or
    /// overwriting it if it does
    WriteFullFile(WriteFullFileOp),
}
