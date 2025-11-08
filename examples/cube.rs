colosseum::run!(Cube);

/// The cube example
struct Cube;

impl colosseum::Game for Cube {
    type Options = CubeOptions;
    type InitialScene = CubeScene;

    const NAME: &str = "Cube Example";
    const COMPANY: &str = "Lance Hart";
    const VERSION: &str = env!("CARGO_PKG_VERSION");
}

/// The command line options to control the cube example
#[derive(argparse::Command)]
#[command(help, version, description = "Displays a controllable cube")]
struct CubeOptions {
    /// The options for controlling colosseum
    #[flag_group]
    colosseum_options: colosseum::ColosseumOptions<Cube>,
}

impl colosseum::GetColosseumOptions<Cube> for CubeOptions {
    fn colosseum_options(&self) -> &colosseum::ColosseumOptions<Cube> {
        &self.colosseum_options
    }
}

/// The main scene which renders a cube
struct CubeScene;

impl colosseum::Scene for CubeScene {
    type Game = Cube;

    fn update(&mut self) {}
}

impl colosseum::InitialScene for CubeScene {
    fn new(
        _: &<Self::Game as colosseum::Game>::Options,
        log_controller: &std::sync::Arc<colosseum::logging::LogController>,
    ) -> Self {
        let logger = log_controller.logger("cube");

        colosseum::info!(logger, "starting main cube scene!");

        CubeScene
    }
}
