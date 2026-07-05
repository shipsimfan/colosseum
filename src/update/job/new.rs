use crate::{
    Error, Result, Window,
    file_io::FileIo,
    logging::Logger,
    render::RenderData,
    update::{InitialScene, Inputs, Scene, UpdateContext, UpdateJob},
};
use alexandria::{
    SlotMap,
    gpu::{VulkanDevice, VulkanFormat},
    math::Vector2u,
};
use std::{marker::PhantomData, time::Duration};

impl<'a, Game: crate::Game> UpdateJob<'a, Game> {
    /// Create a new update job
    pub fn new(
        options: &Game::Options,
        window_size: Vector2u,
        logger: &Logger,
        settings: &'a mut Game::SettingsCache,
        file_io: FileIo,
        window: &Window,

        render_data: &mut RenderData,
        device: &VulkanDevice,
        swapchain_format: VulkanFormat,
    ) -> Result<Option<UpdateJob<'a, Game>>> {
        // Create the initial set of inputs for the game
        let inputs = Inputs::new();

        // Create the pipeline layout that will be used by materials
        let pipeline_layout = device
            .create_pipeline_layout(0, None, &[])
            .map_err(Error::new_inner)?;

        // Create the graphics resources that will be used by the update job and passed to the initial scene
        let mut shaders = SlotMap::new();
        let mut materials = SlotMap::new();

        // Create the update context that will be passed to the initial scene
        let mut update_context = UpdateContext::new(
            Duration::ZERO,
            window_size,
            logger,
            settings,
            &inputs,
            &file_io,
            window,
            render_data,
            device,
            swapchain_format,
            &pipeline_layout,
            &mut shaders,
            &mut materials,
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
            logger: logger.logger("scenes"),
            settings,
            inputs,
            file_io,

            device: device.clone(),
            swapchain_format,
            pipeline_layout,
            shaders,
            materials,
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
