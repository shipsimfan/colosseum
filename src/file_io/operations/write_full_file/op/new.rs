use crate::{
    file_io::{WriteFullFile, WriteFullFileOp},
    single_value_channel,
};
use std::path::PathBuf;

impl WriteFullFileOp {
    /// Create a new [`WriteFullFileOp`] and its corresponding [`WriteFullFile`] handle
    pub fn new(path: PathBuf, data: Vec<u8>) -> (WriteFullFileOp, WriteFullFile) {
        let (result_sender, result_receiver) = single_value_channel::create(false).unwrap();

        (
            WriteFullFileOp {
                result: result_sender,
                path,
                data,
            },
            WriteFullFile {
                result: result_receiver,
            },
        )
    }
}
