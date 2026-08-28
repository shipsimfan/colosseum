use crate::{
    Result,
    file_io::FileIo,
    logging::Logger,
    render::{GpuTransferQueue, RenderJob, Skybox},
    update::{ECS, InitialScene, Inputs, Scene, UpdateContext, UpdateJob, UpdateRenderObjects},
};
use std::marker::PhantomData;

impl<'a, Game: crate::Game> UpdateJob<'a, Game> {
    /// Create a new update job
    pub fn new(
        options: Game::Options,
        logger: &Logger,
        settings: &'a mut Game::SettingsCache,
        file_io: FileIo,

        transfer_queue: GpuTransferQueue,
        render_job: &mut RenderJob,
    ) -> Result<UpdateJob<'a, Game>> {
        Ok(UpdateJob {
            scene: Box::new(EmptyScene(PhantomData)),
            next_scene: Some(Box::new(move |context| {
                <Game::InitialScene as InitialScene>::new(&options, context)
                    .map(|scene| Box::new(scene) as _)
            })),
            first_scene: true,
            logger: logger.logger("scenes"),
            settings,
            inputs: Inputs::new(),
            file_io,
            ecs: ECS::new(logger),
            active_camera: None,
            skybox: Skybox::default(),
            exposure: 1.0,
            contrast: 1.0,
            saturation: 1.0,

            render_objects: UpdateRenderObjects::new(transfer_queue, render_job)?,
        })
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
