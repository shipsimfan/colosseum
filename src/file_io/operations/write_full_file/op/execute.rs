use crate::{Logger, Result, debug, file_io::WriteFullFileOp, warning};
use colosseum_core::new_error;
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
        std::fs::create_dir_all(parent)
            .map_err(|error| new_error!("unable to create \"{}\" - {}", parent.display(), error))?;
    }
    std::fs::write(path, data)
        .map_err(|error| new_error!("unable to write to \"{}\" - {}", path.display(), error))
}
