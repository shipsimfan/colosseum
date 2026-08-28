use crate::settings::DisplaySettings;

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
        }
    }
}
