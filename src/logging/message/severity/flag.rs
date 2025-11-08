use crate::logging::LogSeverity;
use argparse::{DefaultDisplay, Error, Flag};

#[derive(Debug)]
struct UnknownSeverity {
    severity: String,
}

impl std::error::Error for UnknownSeverity {}

impl std::fmt::Display for UnknownSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown severity \"{}\"", self.severity)
    }
}

impl Flag for LogSeverity {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn argparse::ArgumentSource,
        info: &argparse::FlagInfo<Self>,
        long: bool,
    ) -> argparse::Result<()> {
        let argument = match source.next() {
            Some(argument) => argument,
            None => {
                return Err(Error::MissingFlagValue(
                    if long {
                        info.long_name.unwrap()
                    } else {
                        info.short_name.unwrap()
                    },
                    info.value.unwrap_or("VALUE"),
                ));
            }
        };

        let argument = argument.as_str()?.to_lowercase();
        *this = Some(match argument.as_str() {
            "error" => LogSeverity::Error,
            "warning" => LogSeverity::Warning,
            "info" => LogSeverity::Info,
            "debug" => LogSeverity::Debug,
            _ => {
                return Err(Error::InvalidFlagValue(
                    if long {
                        info.long_name.unwrap()
                    } else {
                        info.short_name.unwrap()
                    },
                    info.value.unwrap_or("VALUE"),
                    Box::new(UnknownSeverity { severity: argument }),
                ));
            }
        });

        Ok(())
    }
}

impl DefaultDisplay for LogSeverity {
    type Display<'a>
        = &'a Self
    where
        Self: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self
    }
}
