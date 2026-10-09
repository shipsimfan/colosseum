use crate::error::ErrorMessage;

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}:{})", self.message, self.file, self.line)?;
        Ok(())
    }
}
