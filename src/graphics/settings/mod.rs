use data_format::{Deserialize, Serialize};

mod display_mode;

mod settings_group;

pub use display_mode::DisplayMode;

use crate::graphics::context::AntiAliasing;

/// The settings effecting how the graphics subsystem works
#[derive(Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    /// The x position of the window
    pub x: Option<i32>,

    /// The y position of the window
    pub y: Option<i32>,

    /// The width of the window
    pub width: Option<u32>,

    /// The height of the window
    pub height: Option<u32>,

    /// Should presents be synchronized with vertical blanks?
    #[default(true)]
    pub vsync: bool,

    /// The mode the window should be displayed with
    #[default(DisplayMode::default())]
    pub display_mode: DisplayMode,

    /// The adapter to use for rendering
    pub adapter: Option<String>,

    /// The scale to render internally at, relative to the screen
    #[default(1.0)]
    pub render_scale: f32,

    /// The type of anti-aliasing to use
    pub anti_aliasing: Option<AntiAliasing>,
}
