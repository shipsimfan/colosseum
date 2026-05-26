use crate::{
    Error, ThreadManager, logging::LogController, run::log_metadata::log_metadata,
    settings::SettingsCache,
};
use argparse::Command;
use time::{DateTime, SimpleTimeZone};

mod log_metadata;
mod r#macro;
mod options;

pub use options::*;

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
    let (log_controller, log_start_token) =
        LogController::new(&options.colosseum_options().logging_options)?;

    let init_logger = log_controller.logger("init");
    log_metadata::<Game>(
        &init_logger,
        start_time,
        game_branch,
        game_hash,
        game_build_time,
    )?;

    // Create the thread manager
    let mut thread_manager = ThreadManager::new(&log_controller);

    // Start the logging thread
    log_controller.spawn_thread(log_start_token, &mut thread_manager)?;

    // Load settings and save them back
    let mut settings = <Game::SettingsCache as SettingsCache>::load(
        &options.colosseum_options().settings_path.as_path(),
        log_controller.logger("settings"),
    )?;

    let new_settings = settings.begin_modify();
    settings.save(&new_settings)?;

    // Create the core WSI components

    // Start the job system

    // Start the pacer thread

    // Run the WSI event loop

    // Cleanup all running threads
    if let Err(mut errors) = thread_manager.kill() {
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
