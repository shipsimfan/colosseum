use crate::Error;

impl Error {
    /// Add a message to the existing error
    pub fn add_message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = format!("{} - {}", self.message, message.into()).into();
        self
    }
}
