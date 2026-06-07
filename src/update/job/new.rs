use crate::{
    Result,
    file_io::FileIo,
    logging::Logger,
    render::RenderData,
    update::{InitialScene, Inputs, Scene, UpdateContext, UpdateJob},
};
use alexandria::math::Vector2u;
use std::{marker::PhantomData, time::Duration};

impl<'a, Game: crate::Game> UpdateJob<'a, Game> {
    /// Create a new update job
    pub fn new(
        options: &Game::Options,
        window_size: Vector2u,
        logger: &Logger,
        settings: &'a mut Game::SettingsCache,
        render_data: &mut RenderData,
        file_io: FileIo,
    ) -> Result<Option<UpdateJob<'a, Game>>> {
        let inputs = Inputs::new();

        let mut update_context = UpdateContext::new(
            Duration::ZERO,
            window_size,
            logger,
            settings,
            render_data,
            &inputs,
            &file_io,
        );

        let mut initial_scene: Box<dyn Scene<Game = Game>> = Box::new(
            <Game::InitialScene as InitialScene>::new(options, &mut update_context)?,
        );

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
