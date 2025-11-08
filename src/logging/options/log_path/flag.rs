use crate::logging::LogPath;
use argparse::{DefaultDisplay, Flag};

impl<Game: crate::Game> Flag for LogPath<Game> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn argparse::ArgumentSource,
        info: &argparse::FlagInfo<Self>,
        long: bool,
    ) -> argparse::Result<()> {
        let mut sub_this = None;
        String::parse(&mut sub_this, source, &info.drop_default(), long)?;
        *this = Some(LogPath::Provided(sub_this.unwrap()));
        Ok(())
    }
}

impl<Game: crate::Game> DefaultDisplay for LogPath<Game> {
    type Display<'a>
        = &'a Self
    where
        Self: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self
    }
}
