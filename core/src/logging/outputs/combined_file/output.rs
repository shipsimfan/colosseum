use crate::{
    Error, Result,
    logging::{CombinedFileOutput, LogMessage, LogOutput},
};

impl<Formatter: crate::logging::Formatter> LogOutput for CombinedFileOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) -> Result<()> {
        self.formatter
            .format(message, &mut self.file)
            .map_err(|error| {
                Error::new_with(
                    format!("unable to write to \"{}\"", self.path.display()),
                    error,
                )
            })
    }
}
