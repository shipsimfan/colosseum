use crate::settings::SettingsPath;
use std::marker::PhantomData;

impl<Game: crate::Game> Default for SettingsPath<Game> {
    fn default() -> Self {
        SettingsPath::Default(PhantomData)
    }
}
