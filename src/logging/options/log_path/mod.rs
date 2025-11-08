use std::marker::PhantomData;

mod as_path;
mod display;
mod flag;

/// A path to log files
pub enum LogPath<Game: crate::Game> {
    /// The path was provided
    Provided(String),

    /// The path is default one
    Default(PhantomData<Game>),
}
