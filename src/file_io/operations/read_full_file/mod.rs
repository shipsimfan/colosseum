use crate::{Result, SingleValueReceiver};

mod op;

mod is_complete;
mod result;
mod wait;

pub(in crate::file_io) use op::*;

/// The operation for reading the entire contents of a file into memory
pub struct ReadFullFile {
    /// The location where the result of the read will be stored
    result: SingleValueReceiver<Result<Vec<u8>>>,
}
