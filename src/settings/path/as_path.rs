use crate::settings::SettingsPath;
use std::{borrow::Cow, path::Path};

impl<Game: crate::Game> SettingsPath<Game> {
    /// Get the path
    #[cfg(debug_assertions)]
    pub(crate) fn as_path<'a>(&'a self) -> Cow<'a, Path> {
        match self {
            SettingsPath::Provided(path) => Cow::Borrowed(path),
            SettingsPath::Default(_) => Cow::Borrowed(Path::new("config")),
        }
    }

    /// Get the path
    #[cfg(not(debug_assertions))]
    pub(crate) fn as_path<'a>(&'a self) -> Cow<'a, Path> {
        match self {
            SettingsPath::Provided(path) => Cow::Borrowed(path),
            SettingsPath::Default(_) => {
                Cow::Owned(alexandria::system::config_dir(Game::COMPANY, Game::NAME).unwrap())
            }
        }
    }
}
