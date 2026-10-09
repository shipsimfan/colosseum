use crate::{
    Result,
    logging::{CombinedFileOutput, LogMessage, LogOutput},
    new_error,
};

impl<Formatter: crate::logging::Formatter> LogOutput for CombinedFileOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) -> Result<()> {
        self.formatter
            .format(message, &mut self.file)
            .map_err(|error| {
                new_error!("unable to write to \"{}\" - {}", self.path.display(), error)
            })
    }
}
