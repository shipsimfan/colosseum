use std::borrow::Cow;

mod display;
mod new;

/// A message describing an error
#[derive(Debug)]
pub(in crate::error) struct ErrorMessage {
    /// The human-readable message describing the error
    message: Cow<'static, str>,

    /// The file in which the error occurred
    file: &'static str,

    /// The line number in the file where the error occurred
    line: u32,
}
