use std::{marker::PhantomData, path::PathBuf};

mod as_path;
mod default;
mod display;
mod flag;

/// A path to settings files
pub enum SettingsPath<Game: crate::Game> {
    /// The path was explicitly provided
    Provided(PathBuf),

    /// The path is the default one for the game
    Default(PhantomData<Game>),
}
