use crate::error::InnerError;

impl std::error::Error for InnerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InnerError::ArgParse(error) => Some(error),
            InnerError::Deserialize(_) => None,
            InnerError::IO(error) => Some(error),
            InnerError::Win32(error) => Some(error),
        }
    }
}
