use inner::InnerError;
use std::borrow::Cow;

mod inner;

mod display;
mod new;

/// A result of an Alexandria call
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while running Alexandria
#[derive(Debug)]
pub struct Error {
    /// A human-readable message describing the error
    message: Option<Cow<'static, str>>,

    /// The inner error that caused this error, if any
    inner: Option<InnerError>,
}

impl std::error::Error for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
