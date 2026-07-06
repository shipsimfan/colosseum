use crate::{
    file_io::FileIo,
    logging::Logger,
    render::{Material, Shader},
    update::{ECS, Inputs, Scene},
};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat, VulkanPipelineLayout},
};
use std::sync::Arc;

mod new;
mod run;

/// The persistent state of the update job
pub(crate) struct UpdateJob<'a, Game: crate::Game> {
    /// The current scene being updated
    scene: Box<dyn Scene<Game = Game>>,

    /// The scene to transition to at the start of the next frame, if any
    next_scene: Option<Box<dyn Scene<Game = Game>>>,

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

    /** Render Objects **/

    /// The device to use to create render objects
    device: VulkanDevice,

    /// The format of the swapchain being used
    swapchain_format: VulkanFormat,

    /// The pipeline layout used by materials
    pipeline_layout: VulkanPipelineLayout,

    /// The shaders that have been registered
    shaders: SlotMap<Arc<Shader>>,

    /// The materials that have been registered
    materials: SlotMap<Material>,
}
