use crate::{
    Error, MessageThread, Result, Scene, info,
    logging::LogController,
    settings::SettingsCache,
    util::{expand_environment_string, message_box},
};
use argparse::Command;
use std::path::PathBuf;

mod initial_scene;
mod r#macro;
mod options;
mod running_state;

pub use initial_scene::InitialScene;
pub use options::{ColosseumOptions, GetColosseumOptions};

pub(crate) use running_state::RunningState;

/// Begins the game engine with the provided options, quiting the application based on the result
/// of running
pub fn run<Game: crate::Game>() -> ! {
    if let Err(error) = do_run::<Game>() {
        eprintln!("Error: {}", error);
        message_box("Error", &error.to_string(), None).unwrap();
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

    // Create shared state
    let running_state = RunningState::new();

    // Create logger
    let log_controller = LogController::new(
        &options.colosseum_options().logging_options,
        running_state.clone(),
    )?;
    let init_logger = log_controller.logger("init");
    info!(
        init_logger,
        "starting {} v{} . . .",
        Game::NAME,
        Game::VERSION
    );

    // Load settings
    let settings_path = PathBuf::from(expand_environment_string(
        &options.colosseum_options().settings_path.as_path(),
    )?);
    let mut settings = <Game::SettingsCache as SettingsCache>::load(&settings_path)?;

    // Create window
    let (message_thread, hwnd) = MessageThread::new(
        Game::NAME,
        settings.graphics_settings().clone(),
        &log_controller,
        running_state.clone(),
    )?;

    // Create graphics objects

    // Create scene
    let mut scene = Box::new(Game::InitialScene::new(
        &options,
        &log_controller,
        &mut settings,
    ));

    // Run main loop
    while running_state.is_running() {
        log_controller.frame();
        scene.update();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}
