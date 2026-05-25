use crate::Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message) = &self.message {
            message.fmt(f)?;
        }

        if self.message.is_some() && self.inner.is_some() {
            write!(f, " - ")?;
        }

        if let Some(inner) = &self.inner {
            inner.fmt(f)?;
        }

        Ok(())
    }
}
