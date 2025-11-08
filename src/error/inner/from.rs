use crate::error::InnerError;

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

impl From<argparse::Error> for InnerError {
    fn from(error: argparse::Error) -> Self {
        InnerError::Other(error.to_string())
    }
}

impl<'de> From<json::DeserializeError<'de>> for InnerError {
    fn from(error: json::DeserializeError<'de>) -> Self {
        InnerError::Other(error.to_string())
    }
}
