use crate::{
    DisplaySettings,
    render::{AntiAliasing, ShadowQuality},
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
            anti_aliasing: AntiAliasing::default(),
            shadow_quality: ShadowQuality::default(),
            minimized_frame_rate: 30.0,
            maximum_frame_rate: None,
        }
    }
}
