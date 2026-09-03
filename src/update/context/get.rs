use crate::{
    Result,
    file_io::FileIo,
    logging::Logger,
    render::{Material, MaterialId, Mesh, ShaderId},
    update::{ECS, Inputs, Scene, Skybox, UpdateContext},
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
        self.skybox
    }

    /// Get a mutable reference to the clear color used for this update
    pub fn skybox_mut(&mut self) -> &mut Skybox {
        self.skybox
    }

    /// Get the current set of inputs for the game
    pub fn inputs(&self) -> &Inputs {
        self.inputs
    }

    /// Get a reference to the file I/O system
    pub fn file_io(&self) -> &FileIo {
        self.file_io
    }

    /// Get the default unlit shader
    pub fn default_unlit_shader(&self) -> ShaderId {
        self.render_objects.default_unlit_shader()
    }

    /// Get the default lit shader
    pub fn default_lit_shader(&self) -> ShaderId {
        self.render_objects.default_lit_shader()
    }

    /// Try to get a reference to a material, returning [`None`] if it doesn't exist
    pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
        self.render_objects.get_material(id)
    }

    /// Get a reference to a material
    pub fn material(&self, id: MaterialId) -> &Material {
        self.get_material(id).expect("Material does not exist")
    }

    /// Get the quad primitive
    pub fn quad(&self) -> Id<Mesh> {
        self.render_objects.quad()
    }

    /// Get the plane primitive
    pub fn plane(&self) -> Id<Mesh> {
        self.render_objects.plane()
    }

    /// Get the cube primitive
    pub fn cube(&self) -> Id<Mesh> {
        self.render_objects.cube()
    }

    /// Get the sphere primitive
    pub fn sphere(&self) -> Id<Mesh> {
        self.render_objects.sphere()
    }

    /// Get the cylinder primitive
    pub fn cylinder(&self) -> Id<Mesh> {
        self.render_objects.cylinder()
    }

    /// Take the next scene to switch to, if any
    pub(in crate::update) fn take_next_scene(
        &mut self,
    ) -> Option<Box<dyn FnOnce(&mut UpdateContext<Game>) -> Result<Box<dyn Scene<Game = Game>>>>>
    {
        self.next_scene.take()
    }
}
