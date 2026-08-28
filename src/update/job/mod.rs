use crate::{
    Result,
    file_io::FileIo,
    logging::Logger,
    render::Skybox,
    update::{ECS, Entity, Inputs, Scene, UpdateContext, UpdateRenderObjects},
};
use alexandria::Id;

mod new;
mod run;

/// The persistent state of the update job
pub(crate) struct UpdateJob<'a, Game: crate::Game> {
    /// The current scene being updated
    scene: Box<dyn Scene<Game = Game>>,

    /// The scene to transition to at the start of the next frame, if any
    next_scene:
        Option<Box<dyn FnOnce(&mut UpdateContext<Game>) -> Result<Box<dyn Scene<Game = Game>>>>>,

    /// Is the next scene the first scene of the game?
    first_scene: bool,

    /// A logger to use during the update phase
    logger: Logger,

    /// The game's settings cache
    settings: &'a mut Game::SettingsCache,

    /// The current set of inputs for the game
    inputs: Inputs,

    /// The access for performing asynchronous file I/O operations
    file_io: FileIo,

    /// The ECS system for the game
    ecs: ECS,

    /// The currently active camera
    active_camera: Option<Id<Entity>>,

    /// The skybox to use for rendering the scene
    skybox: Skybox,

    /// The exposure to use for the scene
    exposure: f32,

    /// The contrast to use for the scene
    contrast: f32,

    /// The saturation to use for the scene
    saturation: f32,

    /// The render objects that have been created and are being used by the game
    render_objects: UpdateRenderObjects,
}
