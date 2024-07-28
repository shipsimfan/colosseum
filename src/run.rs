use crate::{event_logger::EventLogger, info, logging::LogController, Scene};
use std::path::Path;

#[cfg(debug_assertions)]
const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

/// Begins running a game with `initial_scene`
pub fn run<F: FnOnce(&LogController) -> Box<dyn Scene>>(
    title: &str,
    log_directory: Option<&Path>,
    initial_scene: F,
) -> ! {
    let exit_code = match do_run(title, log_directory, initial_scene) {
        Ok(()) => 0,
        Err(error) => {
            alexandria::message_box(error.title(), &error);
            1
        }
    };

    std::process::exit(exit_code);
}

/// Actually setups the game engine and runs `initial_scene`
fn do_run<F: FnOnce(&LogController) -> Box<dyn Scene>>(
    title: &str,
    log_directory: Option<&Path>,
    initial_scene: F,
) -> Result<(), Box<dyn alexandria::Error>> {
    // Initial setup
    let log_controller = LogController::new(log_directory);
    let graphics_logger = log_controller.logger("graphics");

    info!(graphics_logger, "Creating graphics instance");
    let instance = alexandria::Instance::new(if DEBUG {
        Some(EventLogger::new(log_controller.logger("vulkan")))
    } else {
        None
    })?;

    info!(graphics_logger, "Creating window");
    let mut window = alexandria::Window::new(title, 1280, 720)?;

    // Main game loop
    let mut scene = initial_scene(&log_controller);
    while window.poll_events() {
        let next_scene = scene.update();
        scene.render();

        if let Some(next_scene) = next_scene {
            scene = next_scene;
        }
    }

    info!(graphics_logger, "Shutting down");

    Ok(())
}
