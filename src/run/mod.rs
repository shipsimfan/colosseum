use crate::{
    Error, ThreadManager, logging::LogController, run::log_metadata::log_metadata,
    settings::SettingsCache,
};
use argparse::Command;
use time::{DateTime, SimpleTimeZone};
use wsi::Wsi;

mod game;
mod log_metadata;
mod r#macro;
mod options;
mod wsi;

pub use options::*;

pub(crate) use wsi::Window;

/// Begins the game engine with the provided options, quiting the application based on the result
/// of running
pub fn run<Game: crate::Game>(
    game_branch: Option<&str>,
    game_hash: Option<&str>,
    game_build_time: Option<&str>,
) -> ! {
    if let Err(error) = do_run::<Game>(game_branch, game_hash, game_build_time) {
        display_error(&error);
        std::process::exit(1);
    }

    std::process::exit(0);
}

/// Begins the game engine with the provided options
fn do_run<Game: crate::Game>(
    game_branch: Option<&str>,
    game_hash: Option<&str>,
    game_build_time: Option<&str>,
) -> Result<(), Error> {
    // Get the start time
    let start_time = DateTime::<SimpleTimeZone>::now_local();

    // Parse arguments
    let options = match <Game::Options as Command>::parse_env() {
        Ok(Some(options)) => options,
        Ok(None) => return Ok(()),
        Err(error) => {
            return Err(Error::new_with("unable to parse arguments", error));
        }
    };

    // Create the logging interface
    let log_controller = LogController::new(&options.colosseum_options().logging_options)?;

    let init_logger = log_controller.logger("init");
    log_metadata::<Game>(
        &init_logger,
        start_time,
        game_branch,
        game_hash,
        game_build_time,
    )?;

    // Create the thread manager
    let thread_manager = ThreadManager::new(&log_controller);

    // Start the logging thread
    log_controller.spawn_thread(
        &thread_manager,
        &options.colosseum_options().logging_options,
    )?;

    // Load settings and save them back
    let mut settings = <Game::SettingsCache as SettingsCache>::load(
        &options.colosseum_options().settings_path.as_path(),
        log_controller.logger("settings"),
    )?;

    // Create the core WSI components
    let (mut wsi, vulkan_instance, surface) = Wsi::new(
        Game::NAME,
        Game::VERSION,
        &init_logger,
        settings.display_settings(),
    )?;
    thread_manager.set_event_queue(wsi.event_queue().clone());

    let new_settings = settings.begin_modify();
    settings.save(&new_settings)?;

    // Start the game thread
    let window = wsi.window();
    thread_manager.spawn(
        "Game".to_string(),
        move |shared_state| {
            game::run::<Game>(
                shared_state,
                vulkan_instance,
                surface,
                settings,
                window,
                init_logger,
                log_controller,
            )
        },
        || {},
    )?;

    // Run the WSI event loop
    let mut error = None;
    while thread_manager.shared_state().is_running() {
        match wsi.pump() {
            Ok(true) => {}
            Ok(false) => break,
            Err(e) => {
                error = Some(e);
                break;
            }
        }
    }

    // Cleanup all running threads
    let mut errors = match (thread_manager.kill(), error) {
        (Ok(()), None) => Vec::new(),
        (Err(errors), None) => errors,
        (Ok(()), Some(error)) => vec![error],
        (Err(mut errors), Some(error)) => {
            errors.push(error);
            errors
        }
    };

    // Print any errors that occured during shutdown, returning the last one as the error for the function
    if errors.len() > 0 {
        let last = errors.len() - 1;
        for error in &errors[..last] {
            display_error(error);
        }

        return Err(errors.swap_remove(last));
    }

    Ok(())
}

fn display_error(error: &Error) {
    eprintln!("Error: {}", error);
}
