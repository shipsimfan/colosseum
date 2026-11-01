use crate::{
    SettingsCache,
    render::{
        Skybox,
        frame_graph::{FrameGraphStructure, SkyboxType},
    },
};

impl FrameGraphStructure {
    /// Create a new [`FrameGraphStructure`]
    pub fn new<Game: crate::Game>(
        settings: &Game::SettingsCache,
        skybox: &Skybox,
    ) -> FrameGraphStructure {
        let display_settings = settings.display_settings();

        let render_scale = display_settings.render_scale() != 1.0;
        let anti_aliasing = display_settings.anti_aliasing();

        let skybox = match skybox {
            Skybox::SolidColor(_) => SkyboxType::SolidColor,
            Skybox::Procedural(_) => SkyboxType::Procedural,
        };

        FrameGraphStructure {
            render_scale,
            anti_aliasing,
            skybox,
        }
    }
}
