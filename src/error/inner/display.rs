use crate::error::InnerError;

impl std::fmt::Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerError::Alexandria(error) => error.fmt(f),
            InnerError::Argparse(error) => error.fmt(f),
            InnerError::IO(error) => error.fmt(f),
            InnerError::Other(error) => error.fmt(f),
        }
    }
}
