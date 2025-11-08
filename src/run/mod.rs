use crate::{Error, Result, info, logging::LogController};
use argparse::Command;
use message_box::message_box;

mod initial_scene;
mod r#macro;
mod message_box;
mod options;

pub use initial_scene::InitialScene;
pub use options::{ColosseumOptions, GetColosseumOptions};

/// Begins the game engine with the provided options, quiting the application based on the result
/// of running
pub fn run<Game: crate::Game>() -> ! {
    if let Err(error) = do_run::<Game>() {
        eprintln!("Error: {}", error);
        message_box("Error", &error.to_string()).unwrap();
        std::process::exit(1);
    }

    std::process::exit(0);
}

/// Begins the game engine with the provided options
fn do_run<Game: crate::Game>() -> Result<()> {
    // Parse arguments
    let options = match <Game::Options as Command>::parse_env() {
        Ok(Some(options)) => options,
        Ok(None) => return Ok(()),
        Err(error) => return Err(Error::new_inner("unable to parse arguments ", error)),
    };

    // Create logger
    let log_controller = LogController::new(&options.colosseum_options().logging_options)?;
    let init_logger = log_controller.logger("init");
    info!(
        init_logger,
        "starting {} v{} . . .",
        Game::NAME,
        Game::VERSION
    );

    // Load settings

    // Create window

    // Create graphics objects

    // Create scene
    let scene = Box::new(Game::InitialScene::new(&options, &log_controller));

    // Run main loop

    Ok(())
}
