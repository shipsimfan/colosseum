use data_format::{Deserialize, Serialize};

mod controller;
mod load_error;
mod save_error;

pub use controller::SettingsController;
pub use load_error::LoadSettingsError;
pub use save_error::SaveSettingsError;

/// Settings which can be saved and loaded to configure the game
pub trait Settings: Default + for<'de> Deserialize<'de> + Serialize {
    /// The name of the settings file, not including extension
    const NAME: &str;
}
