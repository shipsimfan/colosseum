use std::panic::PanicHookInfo;

mod display;
mod error;
mod from;

/// A type of error that can be inside of a [`crate::Error`]
#[derive(Debug)]
pub(crate) enum InnerError {
    /// The error came from alexandria
    Alexandria(alexandria::Error),

    /// The error came from argparse
    Argparse(argparse::Error),

    /// The error came from Rust I/O
    IO(std::io::Error),

    /// The error came from a different source
    Other(String),
}
