use crate::{
    Result,
    logging::{LogMessage, LogOutput, StdoutOutput},
    new_error,
};

impl<Formatter: crate::logging::Formatter> LogOutput for StdoutOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) -> Result<()> {
        self.formatter
            .format(message, &mut self.stdout)
            .map_err(|error| new_error!("unable to write to standard output - {}", error))
    }
}
