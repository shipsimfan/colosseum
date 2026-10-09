use crate::{FileIo, GameOptions, Result, SettingsCache, update::Scene};
use argparse::Command;
use colosseum_core::{Error, LogController, ThreadManager, logger, new_error};
use log_metadata::*;
use time::{DateTime, SimpleTimeZone};

mod game;
mod log_metadata;
mod r#macro;
mod user_event;
mod wsi;

pub(crate) use user_event::*;
pub(crate) use wsi::*;

/// Begins the game engine with the provided options, quiting the application based on the result
/// of running
pub fn run<Game: crate::Game, InitialScene: Scene>(
    initial_scene: InitialScene,
    game_branch: Option<&str>,
    game_hash: Option<&str>,
    game_build_time: Option<&str>,
) -> ! {
    if let Err(error) =
        do_run::<Game, InitialScene>(initial_scene, game_branch, game_hash, game_build_time)
    {
        display_error(&error);
        std::process::exit(1);
    }

    std::process::exit(0);
}

/// Begins the game engine with the provided options
fn do_run<Game: crate::Game, InitialScene: Scene>(
    initial_scene: InitialScene,
    game_branch: Option<&str>,
    game_hash: Option<&str>,
    game_build_time: Option<&str>,
) -> Result<()> {
    // Get the start time
    let start_time = DateTime::<SimpleTimeZone>::now_local();

    // Parse arguments
    let options = match <Game::Options as Command>::parse_env() {
        Ok(Some(options)) => options,
        Ok(None) => return Ok(()),
        Err(error) => {
            return Err(new_error!("unable to parse arguments - {}", error));
        }
    };

    // Create the logging interface
    LogController::new(&options.colosseum_options().logging_options)?;

    let init_logger = logger!("init");
    log_metadata::<Game>(
        &init_logger,
        start_time,
        game_branch,
        game_hash,
        game_build_time,
    )?;

    // Create the thread manager
    let thread_manager = ThreadManager::new()?;

    // Start the file I/O thread
    let file_io = FileIo::new(&thread_manager)?;

    // Load settings and save them back
    let mut settings = <Game::SettingsCache as SettingsCache>::load(
        &options.colosseum_options().settings_path.as_path(),
        logger!("settings"),
        &file_io,
    )?;
    let new_settings = settings.begin_modify();
    settings.save(&new_settings);

    // Create the core WSI components
    let (mut wsi, vulkan_instance, surface, inputs) = Wsi::new(
        Game::NAME,
        Game::VERSION,
        &init_logger,
        settings.display_settings(),
    )?;
    thread_manager.set_event_queue(wsi.event_queue().clone());

    // Start the game thread
    let window = wsi.window(inputs);
    let shared_window = wsi.shared_window().clone();
    let child_thread_manager = thread_manager.clone();
    thread_manager.spawn(
        "Game".to_string(),
        move |shared_state| {
            game::run::<Game, InitialScene>(
                settings,
                options,
                initial_scene,
                window,
                vulkan_instance,
                surface,
                shared_state,
                file_io,
                child_thread_manager,
            )
        },
        move || {
            shared_window.restored_notify().notify().ok();
        },
    )?;

    // Run the WSI event loop
    let mut error = None;
    while thread_manager.is_running() {
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
    let mut errors = match (thread_manager.kill("main"), error) {
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
