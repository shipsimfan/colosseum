use crate::error::InnerError;
use std::error::Error;

impl std::fmt::Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.source().unwrap(), f)
    }
}
