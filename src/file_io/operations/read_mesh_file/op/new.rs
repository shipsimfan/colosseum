use crate::{
    Result,
    file_io::{ReadMeshFile, ReadMeshFileOp},
    single_value_channel,
};
use std::path::PathBuf;

impl ReadMeshFileOp {
    /// Create a new [`ReadMeshFileOp`] and its corresponding [`ReadMeshFile`] handle
    pub fn new(path: PathBuf, notify: bool) -> Result<(ReadMeshFileOp, ReadMeshFile)> {
        let (result_sender, result_receiver) = single_value_channel(notify)?;

        Ok((
            ReadMeshFileOp {
                result: result_sender,
                path,
            },
            ReadMeshFile {
                result: result_receiver,
            },
        ))
    }
}
