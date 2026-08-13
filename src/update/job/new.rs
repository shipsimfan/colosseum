use crate::{
    Result, Window,
    file_io::FileIo,
    logging::Logger,
    render::{FixedRenderObjects, GpuTransferQueue, RenderData, Skybox},
    update::{ECS, InitialScene, Inputs, Scene, UpdateContext, UpdateJob, UpdateRenderObjects},
};
use alexandria::{
    gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanFormat},
    math::Vector2u,
};
use std::{marker::PhantomData, sync::Arc, time::Duration};

impl<'a, Game: crate::Game> UpdateJob<'a, Game> {
    /// Create a new update job
    pub fn new(
        options: &Game::Options,
        window_size: Vector2u,
        logger: &Logger,
        settings: &'a mut Game::SettingsCache,
        file_io: FileIo,
        window: &Window,

        device: VulkanDevice,
        swapchain_format: VulkanFormat,
        transfer_queue: GpuTransferQueue,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
        fixed_render_objects: Arc<FixedRenderObjects>,
        render_data: &mut RenderData,
    ) -> Result<Option<UpdateJob<'a, Game>>> {
        // Create the initial set of inputs for the game
        let inputs = Inputs::new();

        // Create the ECS system for the game
        let mut ecs = ECS::new(logger);

        // Create the render objects
        let mut skybox = Skybox::default();
        let mut render_objects = UpdateRenderObjects::new(
            device,
            swapchain_format,
            transfer_queue,
            memory_properties,
            fixed_render_objects,
        );

        // Create the update context that will be passed to the initial scene
        let mut active_camera = None;
        let mut update_context = UpdateContext::new(
            Duration::ZERO,
            window_size,
            logger,
            settings,
            &inputs,
            &file_io,
            &mut ecs,
            &mut active_camera,
            window,
            &mut skybox,
            render_data,
            &mut render_objects,
        );

        // Create the initial scene for the game
        let mut initial_scene: Box<dyn Scene<Game = Game>> = Box::new(
            <Game::InitialScene as InitialScene>::new(options, &mut update_context)?,
        );

        // Check if the initial scene requested to exit the game or switch to a different scene
        if update_context.should_exit() {
            return Ok(None);
        }
        if let Some(next_scene) = update_context.take_next_scene() {
            initial_scene = next_scene;
        }

        Ok(Some(UpdateJob {
            scene: Box::new(EmptyScene(PhantomData)),
            next_scene: Some(initial_scene),
            first_scene: true,
            logger: logger.logger("scenes"),
            settings,
            inputs,
            file_io,
            ecs,
            active_camera,
            skybox,
            render_objects,
        }))
    }
}

/// An empty scene that does nothing, used as a placeholder until the first scene is activated
struct EmptyScene<Game: crate::Game>(PhantomData<Game>);

impl<Game: crate::Game> Scene for EmptyScene<Game> {
    type Game = Game;

    fn update(&mut self, _: &mut UpdateContext<Game>) -> Result<()> {
        Ok(())
    }
}
