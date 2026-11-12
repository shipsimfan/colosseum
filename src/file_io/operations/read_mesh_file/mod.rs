use crate::{Result, SingleValueReceiver};

mod op;

mod is_complete;
mod result;

pub(in crate::file_io) use op::*;

/// The operation for reading a mesh file into memory
pub(crate) struct ReadMeshFile {
    /// The location where the result of the read will be stored
    result: SingleValueReceiver<Result<parthenon::Mesh>>,
}
