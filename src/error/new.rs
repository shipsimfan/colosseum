use crate::{Error, error::InnerError};
use std::borrow::Cow;

impl Error {
    /// Create a new [`Error`]
    pub(crate) fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Error {
            message: message.into(),
            inner: None,
        }
    }

    /// Create a new [`Error`] caused by a different `inner` error
    pub(crate) fn new_inner<T: Into<Cow<'static, str>>, E: Into<InnerError>>(
        message: T,
        error: E,
    ) -> Self {
        Error {
            message: message.into(),
            inner: Some(error.into()),
        }
    }
}
