use crate::{Error, error::InnerError};
use std::borrow::Cow;

impl Error {
    /// Create a new [`Error`]
    pub(crate) fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Error {
            message: Some(message.into()),
            inner: None,
        }
    }

    /// Create a new [`Error`] caused by a different `inner` error
    pub(crate) fn new_with<T: Into<Cow<'static, str>>, E: Into<InnerError>>(
        message: T,
        error: E,
    ) -> Self {
        Error {
            message: Some(message.into()),
            inner: Some(error.into()),
        }
    }

    /// Create a new [`Error`] caused by a different `inner` error, with no message
    pub(crate) fn new_inner<E: Into<InnerError>>(error: E) -> Self {
        Error {
            message: None,
            inner: Some(error.into()),
        }
    }
}
