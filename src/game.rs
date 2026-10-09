use crate::{GameOptions, settings::SettingsCache};
use colosseum_core::GameMetadata;

/// The definition of common elements to the whole game
pub trait Game: 'static + Sized + GameMetadata {
    /// The command line options the game accepts
    type Options: GameOptions<Self>;

    /// The settings which the game uses
    type SettingsCache: SettingsCache;
}
