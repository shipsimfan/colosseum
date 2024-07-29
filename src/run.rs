use crate::{state::Colosseum, Scene, UpdateContext};
use std::path::Path;

/// Begins running a game with `initial_scene`
pub fn run<F: FnOnce(UpdateContext) -> Box<dyn Scene>>(
    title: &str,
    log_directory: Option<&Path>,
    settings_directory: Option<&Path>,
    initial_scene: F,
) -> ! {
    let exit_code = match do_run(title, log_directory, settings_directory, initial_scene) {
        Ok(()) => 0,
        Err(error) => {
            alexandria::message_box(error.title(), &error);
            1
        }
    };

    std::process::exit(exit_code);
}

fn do_run<F: FnOnce(UpdateContext) -> Box<dyn Scene>>(
    title: &str,
    log_directory: Option<&Path>,
    settings_directory: Option<&Path>,
    initial_scene: F,
) -> Result<(), Box<dyn alexandria::Error>> {
    let mut colosseum = Colosseum::new(title, log_directory, settings_directory)?;
    let initial_scene = initial_scene(colosseum.update_context());

    colosseum.game_loop(initial_scene);

    Ok(())
}
