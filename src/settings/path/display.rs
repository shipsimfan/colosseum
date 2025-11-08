use crate::settings::SettingsPath;

impl<Game: crate::Game> std::fmt::Display for SettingsPath<Game> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_path().fmt(f)
    }
}
