use crate::math::Vector2u;
use alexandria::math::Vector2i;
use data_format::{Deserialize, Serialize};

mod get;
mod set;
mod settings_group;

/// The display settings for the game
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct DisplaySettings {
    /// The position to display the window at, or `None` to let the OS decide
    position: Option<Vector2i>,

    /// The resolution to use for the display, or `None` to let the OS decide
    resolution: Option<Vector2u>,

    /// Whether to use fullscreen mode
    fullscreen: bool,

    /// Whether the window is maximized
    maximized: bool,

    /// The name or UUID of the adapter to use for rendering
    adapter: Option<String>,
}
