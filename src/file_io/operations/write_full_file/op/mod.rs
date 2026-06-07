use crate::{Result, SingleValueSender};
use std::path::PathBuf;

mod execute;
mod new;

/// Writing the entire contents of a file from memory, creating the file if it doesn't exist or
/// overwriting it if it does
pub(in crate::file_io) struct WriteFullFileOp {
    /// The path of the file to write
    path: PathBuf,

    /// The data to write to the file
    data: Vec<u8>,

    /// The place to store the result of the write
    result: SingleValueSender<Result<()>>,
}
