use crate::Error;
use std::borrow::Cow;

impl Error {
    /// Create a new [`Error`]
    pub fn new<T: Into<Cow<'static, str>>>(message: T, file: &'static str, line: u32) -> Self {
        Error {
            messages: Vec::new(),
        }
        .add_message(message, file, line)
    }
}
