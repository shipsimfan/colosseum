use crate::{ReadFullFile, Result, file_io::ReadFullFileOp, single_value_channel};
use std::path::PathBuf;

impl ReadFullFileOp {
    /// Create a new [`ReadFullFileOp`] and its corresponding [`ReadFullFile`] handle
    pub fn new(path: PathBuf, notify: bool) -> Result<(ReadFullFileOp, ReadFullFile)> {
        let (result_sender, result_receiver) = single_value_channel(notify)?;

        Ok((
            ReadFullFileOp {
                result: result_sender,
                path,
            },
            ReadFullFile {
                result: result_receiver,
            },
        ))
    }
}
