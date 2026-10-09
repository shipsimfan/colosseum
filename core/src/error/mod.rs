use message::*;

mod macros;
mod message;

mod add;
mod display;
mod new;

/// A result of an Alexandria call
pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur while running Alexandria
#[derive(Debug)]
pub struct Error {
    /// The messages describing the error
    messages: Vec<ErrorMessage>,
}

impl std::error::Error for Error {}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}
