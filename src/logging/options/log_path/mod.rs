use std::{marker::PhantomData, path::PathBuf};

mod as_path;
mod display;
mod flag;

/// A path to log files
pub enum LogPath<Game: crate::Game> {
    /// The path was explicitly provided
    Provided(PathBuf),

    /// The path is the default one
    Default(PhantomData<Game>),
}
