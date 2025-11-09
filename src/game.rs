use crate::{
    GetColosseumOptions, InitialScene,
    input::{Input, StateTrackingInput},
    settings::SettingsCache,
};

/// The definition of common elements to the whole game
pub trait Game: 'static + Sized {
    /// The command line options the game accepts
    type Options: GetColosseumOptions<Self>;

    /// The settings which the game uses
    type SettingsCache: SettingsCache;

    /// The input to use for the game
    type Input: Input = StateTrackingInput;

    /// The scene to start the game with
    type InitialScene: InitialScene<Game = Self>;

    /// The name of the game, to be used as the title for the window
    const NAME: &str;

    /// The name of the company making the game, used to automatically produce appropriate folders
    const COMPANY: &str;

    /// The version of the game
    ///
    /// Typcially you should set this to `env!("CARGO_PKG_VERSION")`
    const VERSION: &str;
}
