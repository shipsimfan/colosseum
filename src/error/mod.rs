use inner::InnerError;
use std::borrow::Cow;

mod inner;

mod display;
mod error;
mod new;

/// A result of an Alexandria call
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while running Alexandria
#[derive(Debug)]
pub struct Error {
    /// The message giving context to the error
    message: Cow<'static, str>,

    /// The causing error
    inner: Option<InnerError>,
}

impl std::error::Error for Error {}
