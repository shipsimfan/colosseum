use crate::math::Vector2u;
use data_format::{Deserialize, Serialize};

mod get;
mod set;
mod settings_group;

/// The display settings for the game
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DisplaySettings {
    /// The resolution to use for the display
    resolution: Option<Vector2u>,

    /// Whether to use fullscreen mode
    fullscreen: bool,

    /// The name or UUID of the adapter to use for rendering
    adapter: Option<String>,
}
