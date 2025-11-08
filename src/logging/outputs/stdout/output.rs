use crate::logging::{LogMessage, LogOutput, StdoutOutput};

impl<Formatter: crate::logging::Formatter> LogOutput for StdoutOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) {
        self.formatter.format(message, &mut self.stdout).ok();
    }
}
