use crate::{
    Error, Result,
    logging::{LogMessage, LogOutput, StdoutOutput},
};

impl<Formatter: crate::logging::Formatter> LogOutput for StdoutOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) -> Result<()> {
        self.formatter
            .format(message, &mut self.stdout)
            .map_err(|error| Error::new_with("unable to write to standard output", error))
    }
}
