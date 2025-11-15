use crate::{
    Error, MessageThread, Result, Scene, UpdateContext,
    graphics::GraphicsContext,
    input::{Input, InputDevice, InputDeviceKind, InputDeviceMetadata},
    logging::LogController,
    settings::SettingsCache,
    util::{expand_environment_string, message_box},
};
use argparse::Command;
use log_metadata::log_metadata;
use std::{path::PathBuf, rc::Rc, time::Instant};

mod initial_scene;
mod log_metadata;
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
    let start_time = time::DateTime::<time::SimpleTimeZone>::now_local();

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
    log_metadata::<Game>(&init_logger, start_time);

    // Load settings
    let settings_path = PathBuf::from(expand_environment_string(
        &options.colosseum_options().settings_path.as_path(),
    )?);
    let mut settings = <Game::SettingsCache as SettingsCache>::load(&settings_path)?;

    // Create input handler
    let mut input = Game::Input::new();
    let keyboard_id = input.device_connected(InputDevice::new(
        InputDeviceKind::Keyboard,
        "keyboard".into(),
        InputDeviceMetadata::new(0, 0, 0, 1, 6),
        u8::MAX,
        0,
    ));

    // Create window
    let (message_thread, window) = MessageThread::new(
        Game::NAME,
        settings.graphics_settings().clone(),
        &log_controller,
        keyboard_id,
        running_state.clone(),
    )?;
    let message_thread = Rc::new(message_thread);

    // Create graphics context
    let mut graphics_context = GraphicsContext::new(
        window,
        settings.graphics_settings(),
        message_thread.clone(),
        &log_controller,
    )?;

    // Create scene
    let mut last_time = Instant::now();
    let mut scene: Box<dyn Scene<Game = Game>> = Box::new(Game::InitialScene::new(
        &options,
        &mut UpdateContext::new(
            0.0,
            &log_controller,
            &input,
            &mut settings,
            &mut graphics_context,
            &running_state,
        ),
    )?);

    // Run main loop
    while running_state.is_running() {
        // Pre-update actions
        log_controller.frame();
        message_thread.process_inputs(&mut input);

        // Calculate delta time
        let now = Instant::now();
        let delta_t = (now - last_time).as_secs_f32();
        last_time = now;

        // Update
        scene.update(&mut UpdateContext::new(
            delta_t,
            &log_controller,
            &input,
            &mut settings,
            &mut graphics_context,
            &running_state,
        ))?;

        // Render
        graphics_context.render(scene.clear_color())?;
    }

    Ok(())
}
