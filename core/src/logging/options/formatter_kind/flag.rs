use crate::logging::FormatterKind;
use argparse::{DefaultDisplay, Error, Flag};

#[derive(Debug)]
struct UnknownFormatter {
    formatter: String,
}

impl std::error::Error for UnknownFormatter {}

impl std::fmt::Display for UnknownFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown formatter \"{}\"", self.formatter)
    }
}

impl Flag for FormatterKind {
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
            "none" => FormatterKind::None,
            "human" => FormatterKind::Human,
            "json" => FormatterKind::Json,
            "json-pretty" => FormatterKind::JsonPretty,
            _ => {
                return Err(Error::InvalidFlagValue(
                    if long {
                        info.long_name.unwrap()
                    } else {
                        info.short_name.unwrap()
                    },
                    info.value.unwrap_or("VALUE"),
                    Box::new(UnknownFormatter {
                        formatter: argument,
                    }),
                ));
            }
        });

        Ok(())
    }
}

impl DefaultDisplay for FormatterKind {
    type Display<'a>
        = &'a Self
    where
        Self: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self
    }
}
