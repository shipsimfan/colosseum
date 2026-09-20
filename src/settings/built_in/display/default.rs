use crate::{
    render::{AntiAliasingMode, ShadowQuality},
    settings::DisplaySettings,
};

impl Default for DisplaySettings {
    fn default() -> Self {
        DisplaySettings {
            position: None,
            resolution: None,
            fullscreen: false,
            maximized: false,
            adapter: None,
            render_scale: 1.0,
            gamma: 2.2,
            anti_aliasing: AntiAliasingMode::None,
            shadow_quality: ShadowQuality::Medium,
        }
    }
}
