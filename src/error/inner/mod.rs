mod display;
mod error;
mod from;

/// A type of error that can be inside of a [`crate::Error`]
#[derive(Debug)]
pub(crate) enum InnerError {
    /// The error happened during argument parsing
    ArgParse(argparse::Error),

    /// An error occurred while deserializing
    Deserialize(String),

    /// The error came from Rust I/O
    IO(std::io::Error),

    /// The error came from Windows
    Win32(win32::Error),
}
