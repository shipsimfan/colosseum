colosseum::run!(Cube);

/// The cube example
struct Cube;

impl colosseum::Game for Cube {
    type Options = CubeOptions;
    type SettingsCache = CubeSettings;
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

impl colosseum::GameOptions<Cube> for CubeOptions {
    fn colosseum_options(&self) -> &colosseum::ColosseumOptions<Cube> {
        &self.colosseum_options
    }
}

/// The settings cache for the cube example
#[colosseum::settings::settings_cache]
struct CubeSettings {}

/// The initial scene for the cube example
struct CubeScene {
    /// A color that shifts over time to demonstrate the update loop
    color: colosseum::math::ColorHsv<f32, colosseum::math::Linear>,
}

impl colosseum::update::Scene for CubeScene {
    type Game = Cube;

    fn update(
        &mut self,
        context: &mut colosseum::update::UpdateContext<Cube>,
    ) -> colosseum::Result<()> {
        let dt = context.delta_time().as_secs_f32();
        self.color = self.color.shift_hue(dt / 5.0);
        context.set_clear_color(self.color.into_rgb());
        Ok(())
    }
}

impl colosseum::update::InitialScene for CubeScene {
    fn new(
        _: &CubeOptions,
        _: &mut colosseum::update::UpdateContext<Cube>,
    ) -> colosseum::Result<Self> {
        Ok(CubeScene {
            color: colosseum::math::ColorHsv::RED,
        })
    }
}
