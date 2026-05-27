use crate::error::InnerError;

impl From<alexandria::Error> for InnerError {
    fn from(error: alexandria::Error) -> Self {
        InnerError::Alexandria(error)
    }
}

impl From<argparse::Error> for InnerError {
    fn from(error: argparse::Error) -> Self {
        InnerError::Argparse(error)
    }
}

impl From<std::io::Error> for InnerError {
    fn from(error: std::io::Error) -> Self {
        InnerError::IO(error)
    }
}

impl From<String> for InnerError {
    fn from(error: String) -> Self {
        InnerError::Other(error)
    }
}

impl<'de> From<json::DeserializeError<'de>> for InnerError {
    fn from(error: json::DeserializeError<'de>) -> Self {
        InnerError::Other(error.to_string())
    }
}
