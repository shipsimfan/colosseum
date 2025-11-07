colosseum::run!("Cube Example", CubeOptions, CubeScene);

/// The options to control the cube example
#[derive(argparse::Command)]
#[command(help, version)]
pub struct CubeOptions {
    /// The options for colosseum
    #[flag_group]
    colosseum_options: colosseum::ColosseumOptions,
}

impl colosseum::GetColosseumOptions for CubeOptions {
    fn colosseum_options(&self) -> &colosseum::ColosseumOptions {
        &self.colosseum_options
    }
}

/// The main scene which renders a cube
pub struct CubeScene;

impl colosseum::InitialScene for CubeScene {
    type Options = CubeOptions;

    fn new(_: &Self::Options) -> Self {
        CubeScene
    }
}
