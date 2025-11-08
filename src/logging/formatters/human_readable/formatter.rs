use crate::logging::{Formatter, HumanReadableFormatter, LogMessage};

impl Formatter for HumanReadableFormatter {
    const EXTENSION: &str = "log";

    fn format(
        &mut self,
        message: &LogMessage,
        output: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        if self.color {
            write!(output, "\x1B[2m")?;
        }

        write!(
            output,
            "[{}] [{} - {}]  ",
            message.frame(),
            message.scope(),
            message.module()
        )?;

        if self.color {
            write!(output, "\x1B[22;1m{}", message.severity().color())?;
        }

        write!(output, "[{}] ", message.severity())?;

        if self.color {
            write!(output, "\x1B[0m")?;
        }

        writeln!(output, "{}", message.message())
    }
}
