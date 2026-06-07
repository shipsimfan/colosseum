use crate::{Error, Result, debug, file_io::WriteFullFileOp, logging::Logger, warning};
use std::path::Path;

impl WriteFullFileOp {
    /// Execute the file I/O operation
    pub(in crate::file_io::operations) fn execute(self, logger: &Logger) {
        let result = write_file(&self.path, &self.data)
            .map(|_| {
                debug!(
                    logger,
                    "Wrote {} bytes to \"{}\"",
                    self.data.len(),
                    self.path.display()
                );
            })
            .map_err(|error| {
                warning!(logger, "{}", error);
                error
            });
        self.result.send(result).unwrap();
    }
}

fn write_file(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            Error::new_with(format!("unable to create \"{}\"", parent.display()), error)
        })?;
    }
    std::fs::write(path, data).map_err(|error| {
        Error::new_with(format!("unable to write to \"{}\"", path.display()), error)
    })
}
