use crate::settings::SettingsPath;
use std::borrow::Cow;

impl<Game: crate::Game> SettingsPath<Game> {
    /// Get the path
    #[cfg(debug_assertions)]
    pub(crate) fn as_path<'a>(&'a self) -> Cow<'a, str> {
        match self {
            SettingsPath::Provided(path) => Cow::Borrowed(path),
            SettingsPath::Default(_) => Cow::Borrowed("config"),
        }
    }

    /// Get the path
    #[cfg(not(debug_assertions))]
    pub(crate) fn as_path<'a>(&'a self) -> Cow<'a, str> {
        match self {
            SettingsPath::Provided(path) => Cow::Borrowed(path),
            SettingsPath::Default(_) => Cow::Owned(format!(
                "%USERPROFILE%/Documents/My Games/{}/{}/config",
                Game::COMPANY,
                Game::NAME
            )),
        }
    }
}
