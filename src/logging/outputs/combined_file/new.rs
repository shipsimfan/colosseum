use crate::{Error, Result, logging::CombinedFileOutput};
use std::path::Path;

impl<Formatter: crate::logging::Formatter> CombinedFileOutput<Formatter> {
    /// Open a file under `path` for combined file logging
    pub fn new(path: &Path, mut formatter: Formatter) -> Result<Self> {
        let path = path.join(format!("all.{}", Formatter::EXTENSION));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|error| {
                Error::new_inner(format!("unable to open \"{}\"", path.display()), error)
            })?;

        formatter.start(&mut file).map_err(|error| {
            Error::new_inner(format!("unable to write to \"{}\"", path.display()), error)
        })?;

        Ok(CombinedFileOutput { file, formatter })
    }
}
