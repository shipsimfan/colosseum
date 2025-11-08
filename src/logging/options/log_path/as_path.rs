use crate::logging::LogPath;
use std::borrow::Cow;

impl<Game: crate::Game> LogPath<Game> {
    /// Get the path
    #[cfg(debug_assertions)]
    pub(in crate::logging) fn as_path<'a>(&'a self) -> Cow<'a, str> {
        match self {
            LogPath::Provided(path) => Cow::Borrowed(path),
            LogPath::Default(_) => Cow::Borrowed("logs"),
        }
    }

    /// Get the path
    #[cfg(not(debug_assertions))]
    pub(in crate::logging) fn as_path<'a>(&'a self) -> Cow<'a, str> {
        match self {
            LogPath::Provided(path) => Cow::Borrowed(path),
            LogPath::Default(_) => Cow::Owned(format!(
                "%LOCALAPPDATA%/{}/{}/logs",
                Game::COMPANY,
                Game::NAME
            )),
        }
    }
}
