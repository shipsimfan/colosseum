use crate::logging::{LogMessage, LogOutput, ScopeFilesOutput, outputs::scope_files::ScopeFile};

impl<Formatter: crate::logging::Formatter> LogOutput for ScopeFilesOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) {
        for file in &mut self.files {
            if file.scope() == message.scope() {
                file.output(message);
                return;
            }
        }

        if let Ok(mut file) =
            ScopeFile::open(&self.path, message.scope(), self.base_formatter.clone())
        {
            file.output(message);
            self.files.push(file);
        }
    }
}
