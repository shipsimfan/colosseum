use crate::{
    Result,
    logging::{LogMessage, LogOutput, ScopeFilesOutput, outputs::scope_files::ScopeFile},
};

impl<Formatter: crate::logging::Formatter> LogOutput for ScopeFilesOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) -> Result<()> {
        for file in &mut self.files {
            if file.scope() == message.scope() {
                return file.output(message);
            }
        }

        if let Ok(mut file) =
            ScopeFile::open(&self.path, message.scope(), self.base_formatter.clone())
        {
            file.output(message)?;
            self.files.push(file);
        }

        Ok(())
    }
}
