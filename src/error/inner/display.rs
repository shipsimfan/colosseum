use crate::error::InnerError;

impl std::fmt::Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerError::IO(error) => error.fmt(f),
            InnerError::Win32(error) => error.fmt(f),
            InnerError::Other(error) => error.fmt(f),
        }
    }
}
