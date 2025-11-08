use crate::error::InnerError;

impl std::error::Error for InnerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            InnerError::ArgParse(error) => error,
            InnerError::IO(error) => error,
            InnerError::Win32(error) => error,
        })
    }
}
