use crate::logging::{LogMessage, outputs::scope_files::ScopeFile};

impl<Formatter: crate::logging::Formatter> ScopeFile<Formatter> {
    /// Output `message` to this file
    pub fn output(&mut self, message: &LogMessage) {
        self.formatter.format(message, &mut self.file).ok();
    }
}
