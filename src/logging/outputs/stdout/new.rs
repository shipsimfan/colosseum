use crate::{Error, Result, logging::StdoutOutput};
use std::io::stdout;

impl<Formatter: crate::logging::Formatter> StdoutOutput<Formatter> {
    /// Create a new [`StdoutOutput`]
    pub fn new(mut formatter: Formatter) -> Result<Self> {
        let mut stdout = stdout();

        formatter
            .start(&mut stdout)
            .map_err(|error| Error::new_inner("unable to write to stdout", error))?;

        Ok(StdoutOutput { stdout, formatter })
    }
}
