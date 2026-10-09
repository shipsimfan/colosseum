use crate::logging::StdoutOutput;

impl<Formatter: crate::logging::Formatter> Drop for StdoutOutput<Formatter> {
    fn drop(&mut self) {
        self.formatter.end(&mut self.stdout).ok();
    }
}
