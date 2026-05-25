use crate::logging::{Formatter, JsonFormatter, LogMessage};

impl Formatter for JsonFormatter {
    const EXTENSION: &str = "json";

    fn start(&mut self, output: &mut dyn std::io::Write) -> std::io::Result<()> {
        writeln!(output, "[")
    }

    fn format(
        &mut self,
        message: &LogMessage,
        output: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        if self.first {
            self.first = false;
        } else {
            writeln!(output, ",")?;
        }

        match if self.pretty {
            json::to_write_pretty(&message, &mut *output)
        } else {
            json::to_write(&message, &mut *output)
        } {
            Ok(()) => {}
            Err(json::SerializeError::IO(error)) => return Err(error),
            Err(error) => panic!("unable to serialize message - {}", error),
        }

        Ok(())
    }

    fn end(&mut self, output: &mut dyn std::io::Write) -> std::io::Result<()> {
        if !self.first {
            writeln!(output)?;
        }
        writeln!(output, "]")
    }
}
