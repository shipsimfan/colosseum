use data_format::{Deserialize, Serialize};

mod controller;

pub use controller::SettingsController;

/// Settings which can be saved and loaded to configure the game
pub trait Settings: Default + for<'de> Deserialize<'de> + Serialize {
    /// The name of the settings file, not including extension
    const NAME: &str;
}
