use crate::{Logger, debug, file_io::ReadFullFileOp, warning};
use colosseum_core::new_error;

impl ReadFullFileOp {
    /// Execute the file I/O operation
    pub(in crate::file_io::operations) fn execute(self, logger: &Logger) {
        let result = std::fs::read(&self.path)
            .map(|contents| {
                debug!(
                    logger,
                    "Read {} bytes from \"{}\"",
                    contents.len(),
                    self.path.display()
                );
                contents
            })
            .map_err(|error| {
                warning!(
                    logger,
                    "Unable to read from \"{}\" - {}",
                    self.path.display(),
                    error
                );
                new_error!(
                    "unable to read from \"{}\" - {}",
                    self.path.display(),
                    error,
                )
            });
        self.result.send(result).unwrap();
    }
}
