use crate::{Result, SingleValueSender};
use std::path::PathBuf;

mod execute;
mod new;

/// Reading a mesh file into memory
pub(in crate::file_io) struct ReadMeshFileOp {
    /// The path of the file to read
    path: PathBuf,

    /// The place to store the result of the read
    result: SingleValueSender<Result<parthenon::Mesh>>,
}
