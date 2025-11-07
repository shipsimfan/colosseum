mod initial_scene;
mod r#macro;
mod options;

pub use initial_scene::InitialScene;
pub use options::{ColosseumOptions, GetColosseumOptions};

/// Begins the game engine with the provided options
pub fn run<Scene: InitialScene<Options = Options>, Options: GetColosseumOptions>(title: &str) -> ! {
    // Parse arguments
    let options = match Options::parse_env() {
        Ok(Some(options)) => options,
        Ok(None) => std::process::exit(0),
        Err(error) => {
            eprintln!("Error: unable to parse arguments - {}", error);
            // TODO: Display a message box
            std::process::exit(1);
        }
    };

    // Create logger

    // Load settings

    // Create window

    // Create graphics objects

    // Create scene
    let scene = Box::new(Scene::new(&options));

    // Run main loop

    // Shutdown threads and join

    std::process::exit(0)
}
