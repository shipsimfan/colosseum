use crate::logging::{CombinedFileOutput, LogMessage, LogOutput};

impl<Formatter: crate::logging::Formatter> LogOutput for CombinedFileOutput<Formatter> {
    fn output(&mut self, message: &LogMessage) {
        self.formatter.format(message, &mut self.file).ok();
    }
}
