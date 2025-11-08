use std::marker::PhantomData;

mod as_path;
mod display;
mod flag;

/// A path to settings files
pub enum SettingsPath<Game: crate::Game> {
    /// The path was provided
    Provided(String),

    /// The path is default one
    Default(PhantomData<Game>),
}
