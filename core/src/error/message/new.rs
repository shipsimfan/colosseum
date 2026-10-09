use crate::error::ErrorMessage;
use std::borrow::Cow;

impl ErrorMessage {
    /// Create a new [`ErrorMessage`]
    pub(in crate::error) fn new<T: Into<Cow<'static, str>>>(
        message: T,
        file: &'static str,
        line: u32,
    ) -> ErrorMessage {
        ErrorMessage {
            message: message.into(),
            file,
            line,
        }
    }
}
