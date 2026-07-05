use crate::{
    Window,
    file_io::FileIo,
    logging::Logger,
    render::{Material, RenderData, Shader},
    update::{Inputs, Scene},
};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat, VulkanPipelineLayout},
    math::Vector2u,
};
use std::{sync::Arc, time::Duration};

mod add;
mod create;
mod get;
mod new;
mod remove;
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
    next_scene: Option<Box<dyn Scene<Game = Game>>>,

    /// The current set of inputs for the game
    inputs: &'a Inputs,

    /// The access for performing asynchronous file I/O operations
    file_io: &'a FileIo,

    /// The window being rendered into
    window: &'a Window,

    /** Render Objects **/

    /// The data to be sent to the render job for rendering
    render_data: &'a mut RenderData,

    /// The device used to create render objects
    device: &'a VulkanDevice,

    /// The format of the swapchain being used
    swapchain_format: VulkanFormat,

    /// The pipeline layout used by materials
    pipeline_layout: &'a VulkanPipelineLayout,

    /// The shaders that have been registered
    shaders: &'a mut SlotMap<Arc<Shader>>,

    /// The materials that have been registered
    materials: &'a mut SlotMap<Material>,
}
