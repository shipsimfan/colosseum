use crate::logging::LogPath;
use std::{borrow::Cow, path::Path};

impl<Game: crate::Game> LogPath<Game> {
    /// Get the path
    #[cfg(debug_assertions)]
    pub(in crate::logging) fn as_path<'a>(&'a self) -> Cow<'a, Path> {
        match self {
            LogPath::Provided(path) => Cow::Borrowed(path),
            LogPath::Default(_) => Cow::Borrowed(Path::new("logs")),
        }
    }

    /// Get the path
    #[cfg(not(debug_assertions))]
    pub(in crate::logging) fn as_path<'a>(&'a self) -> Cow<'a, Path> {
        match self {
            LogPath::Provided(path) => Cow::Borrowed(path),
            LogPath::Default(_) => {
                Cow::Owned(alexandria::system::logs_dir(Game::COMPANY, Game::NAME).unwrap())
            }
        }
    }
}
