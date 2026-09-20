use crate::{
    math::Vector2u,
    render::{AntiAliasingMode, ShadowQuality},
};
use alexandria::math::Vector2i;
use data_format::{Deserialize, Serialize};

mod default;
mod get;
mod set;
mod settings_group;

/// The display settings for the game
#[derive(Clone, Serialize, Deserialize)]
pub struct DisplaySettings {
    /// The position to display the window at, or `None` to let the OS decide
    position: Option<Vector2i>,

    /// The resolution to use for the display, or `None` to let the OS decide
    resolution: Option<Vector2u>,

    /// Whether to use fullscreen mode
    #[default(false)]
    fullscreen: bool,

    /// Whether the window is maximized
    #[default(false)]
    maximized: bool,

    /// The name or UUID of the adapter to use for rendering
    adapter: Option<String>,

    /// The render scale to use for rendering
    #[default(1.0)]
    render_scale: f32,

    /// The gamma to use for rendering
    #[default(2.2)]
    gamma: f32,

    /// The anti-aliasing mode to use for rendering
    #[default(AntiAliasingMode::None)]
    anti_aliasing: AntiAliasingMode,

    /// The quality for shadows to use
    #[default(ShadowQuality::Medium)]
    shadow_quality: ShadowQuality,
}
