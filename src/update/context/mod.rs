use crate::{
    Result, Window,
    file_io::FileIo,
    logging::Logger,
    render::{RenderData, Skybox},
    update::{ECS, Entity, Inputs, Scene, UpdateRenderObjects},
};
use alexandria::{Id, math::Vector2u};
use std::time::Duration;

mod complete;
mod create;
mod execute;
mod get;
mod new;
mod remove;
mod scene_reset;
mod set;

/// The context passed to scenes during the update phase of the game loop
pub struct UpdateContext<'a, Game: crate::Game> {
    /// The delta time since the last update
    delta_time: Duration,

    /// The current size of the window
    window_size: Vector2u,

    /// A logger so the game can create its own loggers
    logger: &'a Logger,

    /// The current settings cache, so the game can read settings
    settings: &'a mut Game::SettingsCache,

    /// Whether the game should exit after this update
    should_exit: bool,

    /// The next scene to switch to, if any
    next_scene:
        Option<Box<dyn FnOnce(&mut UpdateContext<Game>) -> Result<Box<dyn Scene<Game = Game>>>>>,

    /// The current set of inputs for the game
    inputs: &'a Inputs,

    /// The access for performing asynchronous file I/O operations
    file_io: &'a FileIo,

    /// The ECS system for the game
    ecs: &'a mut ECS,

    /// The index of the currently active camera
    active_camera: &'a mut Option<Id<Entity>>,

    /// The window being rendered into
    window: &'a Window,

    /// The skybox currently set
    skybox: &'a mut Skybox,

    /// The data to be sent to the render job for rendering
    render_data: &'a mut RenderData,

    /// The render objects that have been created and are being used by the game
    render_objects: &'a mut UpdateRenderObjects,
}
