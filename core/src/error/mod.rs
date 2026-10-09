use std::borrow::Cow;

mod add;
mod display;
mod new;

/// A result of an Alexandria call
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while running Alexandria
#[derive(Debug)]
pub struct Error {
    /// A human-readable message describing the error
    message: Cow<'static, str>,
}

impl std::error::Error for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
