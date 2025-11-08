use crate::error::InnerError;

impl From<argparse::Error> for InnerError {
    fn from(error: argparse::Error) -> Self {
        InnerError::ArgParse(error)
    }
}

impl From<std::io::Error> for InnerError {
    fn from(error: std::io::Error) -> Self {
        InnerError::IO(error)
    }
}

impl From<win32::Error> for InnerError {
    fn from(error: win32::Error) -> Self {
        InnerError::Win32(error)
    }
}
