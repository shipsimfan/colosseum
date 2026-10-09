use crate::Error;
use std::borrow::Cow;

impl Error {
    /// Create a new [`Error`]
    pub fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Error {
            message: message.into(),
        }
    }

    /// Create a new [`Error`] caused by a different `inner` error
    pub(crate) fn new_with<T: Into<Cow<'static, str>>, E: std::error::Error>(
        message: T,
        error: E,
    ) -> Self {
        Error::new(format!("{} - {}", message.into(), error))
    }

    /// Create a new [`Error`] caused by a different `inner` error, with no message
    pub(crate) fn new_error<E: std::error::Error>(error: E) -> Self {
        Error::new(error.into().to_string())
    }
}
