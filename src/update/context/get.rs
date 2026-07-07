use crate::{
    file_io::FileIo,
    logging::Logger,
    render::{Material, RenderData, Skybox},
    update::{ECS, Inputs, Scene, UpdateContext},
};
use alexandria::{Id, math::Vector2u};
use std::time::Duration;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Get the delta time since the last update
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    /// Get the current size of the window
    pub fn window_size(&self) -> Vector2u {
        self.window_size
    }

    /// Create a new logger with the given scope
    pub fn logger(&self, scope: &'static str) -> Logger {
        self.logger.logger(scope)
    }

    /// Get a reference to the settings cache
    pub fn settings(&self) -> &Game::SettingsCache {
        self.settings
    }

    /// Get if the game should exit after this update
    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    /// Get a reference to the ECS system
    pub fn ecs(&self) -> &ECS {
        &self.ecs
    }

    /// Get a mutable reference to the ECS system
    pub fn ecs_mut(&mut self) -> &mut ECS {
        &mut self.ecs
    }

    /// Get a reference to the skybox used for this update
    pub fn skybox(&self) -> &Skybox {
        &self.render_data.skybox
    }

    /// Get a mutable reference to the clear color used for this update
    pub fn skybox_mut(&mut self) -> &mut Skybox {
        &mut self.render_data.skybox
    }

    /// Get a reference to a registered [`Material`]
    pub fn material(&self, material: Id<Material>) -> &Material {
        &self.materials[material]
    }

    /// Get a mutable reference to a registered [`Material`]
    pub fn material_mut(&mut self, material: Id<Material>) -> &mut Material {
        &mut self.materials[material]
    }

    /// Get the current set of inputs for the game
    pub fn inputs(&self) -> &Inputs {
        self.inputs
    }

    /// Get a reference to the file I/O system
    pub fn file_io(&self) -> &FileIo {
        self.file_io
    }

    /// Get the render data for this update
    pub(in crate::update) fn render_data(&mut self) -> &mut RenderData {
        self.render_data
    }

    /// Take the next scene to switch to, if any
    pub(in crate::update) fn take_next_scene(&mut self) -> Option<Box<dyn Scene<Game = Game>>> {
        self.next_scene.take()
    }
}
