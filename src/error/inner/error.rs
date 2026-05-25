use crate::error::InnerError;

impl std::error::Error for InnerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InnerError::Argparse(error) => Some(error),
            InnerError::IO(error) => Some(error),
            InnerError::Other(_) => None,
        }
    }
}
