use crate::math::Vector2u;
use data_format::{Deserialize, Serialize};

mod settings_group;

/// The display settings for the game
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DisplaySettings {
    /// The resolution to use for the display
    pub resolution: Option<Vector2u>,

    /// Whether to use fullscreen mode
    pub fullscreen: bool,
}
