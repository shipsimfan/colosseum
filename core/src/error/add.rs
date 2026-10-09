use crate::{Error, error::ErrorMessage};
use std::borrow::Cow;

impl Error {
    /// Add a message to the existing error
    pub fn add_message<T: Into<Cow<'static, str>>>(
        mut self,
        message: T,
        file: &'static str,
        line: u32,
    ) -> Self {
        self.messages.push(ErrorMessage::new(message, file, line));
        self
    }
}
