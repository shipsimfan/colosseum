use crate::{Error, debug, file_io::ReadFullFileOp, logging::Logger, warning};

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
                Error::new_with(
                    format!("unable to read from \"{}\"", self.path.display()),
                    error,
                )
            });
        self.result.send(result).unwrap();
    }
}
