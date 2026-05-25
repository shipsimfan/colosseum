use crate::{
    Error, Result,
    logging::{LogMessage, outputs::scope_files::ScopeFile},
};

impl<Formatter: crate::logging::Formatter> ScopeFile<Formatter> {
    /// Output `message` to this file
    pub fn output(&mut self, message: &LogMessage) -> Result<()> {
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
