colosseum::run!(Cube);

colosseum::render::compile_shader!(
    /// A shader which renders a single triangle
    pub const TRIANGLE_SHADER = "triangle.slang",
    vert_main,
    frag_main
);

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

    /// A count of the number of frames that have been rendered
    frames: usize,

    /// The amount of time that has passed since the last display of FPS
    fps_timer: f32,

    /// The logger for the cube example
    logger: colosseum::logging::Logger,

    /// The material used to render the cube
    material: colosseum::Id<colosseum::render::Material>,

    /// Should the cube be rendered?
    render_state: bool,
}

impl colosseum::update::Scene for CubeScene {
    type Game = Cube;

    fn update(
        &mut self,
        context: &mut colosseum::update::UpdateContext<Cube>,
    ) -> colosseum::Result<()> {
        // Shift the color over time based on user input
        let amount = context.delta_time().as_secs_f32() / 5.0;
        self.color = if context.inputs().key(colosseum::Key::Left)
            || context.inputs().key(colosseum::Key::A)
        {
            self.color.add_hue(amount)
        } else if context.inputs().key(colosseum::Key::Right)
            || context.inputs().key(colosseum::Key::D)
        {
            self.color.sub_hue(amount)
        } else {
            self.color
        };

        context.set_skybox(self.color.into_rgb());

        // Toggle fullscreen mode when the user presses F11
        if context.inputs().key_down(colosseum::Key::F11) {
            if context.settings().display().fullscreen() {
                context.unset_fullscreen()?;
            } else {
                context.set_fullscreen()?;
            }
        }

        // Render the cube
        if context.inputs().key_down(colosseum::Key::V) {
            self.render_state = !self.render_state;
        }
        if self.render_state {
            context.add_renderable(self.material);
        }

        // Display the FPS every second
        self.frames += 1;
        self.fps_timer += context.delta_time().as_secs_f32();
        if self.fps_timer >= 1.0 {
            colosseum::info!(self.logger, "FPS: {}", self.frames);
            self.frames = 0;
            self.fps_timer = self.fps_timer.fract();
        }

        Ok(())
    }
}

impl colosseum::update::InitialScene for CubeScene {
    fn new(
        _: &CubeOptions,
        context: &mut colosseum::update::UpdateContext<Cube>,
    ) -> colosseum::Result<Self> {
        let shader = context.create_shader(&TRIANGLE_SHADER)?;
        let material = context.create_material(shader)?;

        Ok(CubeScene {
            color: colosseum::math::ColorHsv::RED,
            frames: 0,
            fps_timer: 0.0,
            logger: context.logger("cube"),
            material,
            render_state: true,
        })
    }
}
