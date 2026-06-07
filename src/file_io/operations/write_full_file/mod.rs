use crate::{Result, SingleValueReceiver};

mod op;

mod is_complete;
mod result;

pub(in crate::file_io) use op::*;

/// Writes a full file to disk, creating it if it doesn't exist and overwriting it if it does
pub struct WriteFullFile {
    /// The location where the result of the write will be stored
    result: SingleValueReceiver<Result<()>>,
}
