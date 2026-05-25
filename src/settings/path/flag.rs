use crate::settings::SettingsPath;
use argparse::{DefaultDisplay, Flag};
use std::path::PathBuf;

impl<Game: crate::Game> Flag for SettingsPath<Game> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn argparse::ArgumentSource,
        info: &argparse::FlagInfo<Self>,
        long: bool,
    ) -> argparse::Result<()> {
        let mut sub_this = None;
        PathBuf::parse(&mut sub_this, source, &info.drop_default(), long)?;
        *this = Some(SettingsPath::Provided(sub_this.unwrap()));
        Ok(())
    }
}

impl<Game: crate::Game> DefaultDisplay for SettingsPath<Game> {
    type Display<'a>
        = &'a Self
    where
        Self: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self
    }
}
