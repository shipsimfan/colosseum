use crate::Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.messages.len() {
            write!(f, "{}", self.messages[i])?;

            if i < self.messages.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
