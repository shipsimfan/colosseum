use crate::Scene;
use std::borrow::Cow;

#[cfg(debug_assertions)]
const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

/// Begins running a game with `initial_scene`
pub fn run<S: Scene>(title: &str, initial_scene: S) -> ! {
    let exit_code = match do_run(title, Box::new(initial_scene)) {
        Ok(()) => 0,
        Err(error) => {
            alexandria::message_box(error.title(), &error);
            1
        }
    };

    std::process::exit(exit_code);
}

/// Actually setups the game engine and runs `initial_scene`
fn do_run(title: &str, mut scene: Box<dyn Scene>) -> Result<(), Box<dyn alexandria::Error>> {
    // Initial setup
    let instance = alexandria::Instance::new(if DEBUG { Some(log_callback) } else { None })?;
    let mut window = alexandria::Window::new(title, 1280, 720)?;

    // Main game loop
    while window.poll_events() {
        let next_scene = scene.update();
        scene.render();

        if let Some(next_scene) = next_scene {
            scene = next_scene;
        }
    }

    Ok(())
}

fn log_callback(severity: alexandria::Severity, message: &str, objects: Vec<Cow<str>>) {
    eprint!("[{}] {}", severity, message);
    if objects.len() > 0 {
        eprint!(" ({:?})", objects);
    }
    eprintln!();
}
