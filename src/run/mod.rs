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
pub fn run<Game: crate::Game>(game_hash: Option<&str>, game_build_time: Option<&str>) -> ! {
    if let Err(error) = do_run::<Game>(game_hash, game_build_time) {
        eprintln!("Error: {}", error);
        message_box("Error", &error.to_string(), None).unwrap();
        std::process::exit(1);
    }

    std::process::exit(0);
}

/// Begins the game engine with the provided options
fn do_run<Game: crate::Game>(game_hash: Option<&str>, game_build_time: Option<&str>) -> Result<()> {
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
    log_metadata::<Game>(&init_logger, start_time, game_hash, game_build_time);

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
    let (mut graphics_context, mut managed_objects) = GraphicsContext::new(
        window,
        settings.graphics_settings(),
        message_thread.clone(),
        &log_controller,
    )?;

    // Create scene
    let mut last_time = Instant::now();
    let mut init_update_context = UpdateContext::new(
        0.0,
        &log_controller,
        &input,
        &mut settings,
        &mut graphics_context,
        &mut managed_objects,
        &running_state,
    );
    let mut scene: Box<dyn Scene<Game = Game>> =
        Box::new(Game::InitialScene::new(&options, &mut init_update_context)?);
    activate_scene(&mut scene, &mut init_update_context);

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
        let mut update_context = UpdateContext::new(
            delta_t,
            &log_controller,
            &input,
            &mut settings,
            &mut graphics_context,
            &mut managed_objects,
            &running_state,
        );
        scene.update(&mut update_context)?;
        let next_scene = update_context.take_next_scene();

        // Render
        graphics_context.render(&mut managed_objects, scene.clear_color(), delta_t)?;

        // Change scene
        if let Some(next_scene) = next_scene {
            // Deactivate old scene
            scene.on_deactivate(&mut UpdateContext::new(
                delta_t,
                &log_controller,
                &input,
                &mut settings,
                &mut graphics_context,
                &mut managed_objects,
                &running_state,
            ));
            drop(scene);

            // Clear scene-lifetime arenas
            managed_objects.transforms.clear();
            managed_objects.graphics.cameras.clear();
            managed_objects.graphics.mesh_renderers.clear();
            managed_objects.graphics.lights.directional.clear();
            managed_objects.graphics.lights.point.clear();
            managed_objects.graphics.lights.spot.clear();

            // Activate next scene
            let mut activate_update_context = &mut UpdateContext::new(
                delta_t,
                &log_controller,
                &input,
                &mut settings,
                &mut graphics_context,
                &mut managed_objects,
                &running_state,
            );
            scene = next_scene(&mut activate_update_context);
            activate_scene(&mut scene, &mut activate_update_context);
        }
    }

    Ok(())
}

fn activate_scene<Game: crate::Game>(
    scene: &mut Box<dyn Scene<Game = Game>>,
    context: &mut UpdateContext<Game>,
) {
    let (ambient_color, ambient_intensity) = scene.init_ambient();
    context.graphics.lights.ambient.set_color(ambient_color);
    context
        .graphics
        .lights
        .ambient
        .set_intensity(ambient_intensity);
    scene.on_active(context);
}
