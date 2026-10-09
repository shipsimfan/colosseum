use crate::{
    Result,
    logging::{LogMessage, outputs::scope_files::ScopeFile},
    new_error,
};

impl<Formatter: crate::logging::Formatter> ScopeFile<Formatter> {
    /// Output `message` to this file
    pub fn output(&mut self, message: &LogMessage) -> Result<()> {
        self.formatter
            .format(message, &mut self.file)
            .map_err(|error| {
                new_error!("unable to write to \"{}\" - {}", self.path.display(), error)
            })
    }
}
