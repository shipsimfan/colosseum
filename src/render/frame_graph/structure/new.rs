use crate::{SettingsCache, render::frame_graph::FrameGraphStructure};

impl FrameGraphStructure {
    /// Create a new [`FrameGraphStructure`]
    pub fn new<Game: crate::Game>(settings: &Game::SettingsCache) -> FrameGraphStructure {
        let display_settings = settings.display_settings();

        let render_scale = display_settings.render_scale() != 1.0;
        let anti_aliasing = display_settings.anti_aliasing();

        FrameGraphStructure {
            render_scale,
            anti_aliasing,
        }
    }
}
