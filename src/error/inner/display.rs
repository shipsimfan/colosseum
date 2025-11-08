use crate::error::InnerError;

impl std::fmt::Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerError::ArgParse(error) => error.fmt(f),
            InnerError::Deserialize(error) => error.fmt(f),
            InnerError::IO(error) => error.fmt(f),
            InnerError::Win32(error) => error.fmt(f),
        }
    }
}
