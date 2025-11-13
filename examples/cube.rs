#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

impl colosseum::GetColosseumOptions<Cube> for CubeOptions {
    fn colosseum_options(&self) -> &colosseum::ColosseumOptions<Cube> {
        &self.colosseum_options
    }
}

#[colosseum::settings::settings_cache]
struct CubeSettings {}

/// The main scene which renders a cube
struct CubeScene {
    /// The logger to print messages with
    logger: colosseum::logging::Logger,

    /// The amount of time that has passed since last second
    second_time: f32,

    /// The number of frames that have happened
    frames: u32,

    /// The camera the scene is rendered with
    #[allow(unused)]
    camera: colosseum::graphics::Camera,

    /// The renderer for the cube
    #[allow(unused)]
    mesh_renderer: colosseum::graphics::MeshRenderer,
}

impl colosseum::Scene for CubeScene {
    type Game = Cube;

    fn update(
        &mut self,
        context: &mut colosseum::UpdateContext<Self::Game>,
    ) -> colosseum::Result<()> {
        self.frames += 1;
        self.second_time += context.delta_t();

        while self.second_time >= 1.0 {
            colosseum::info!(self.logger, "FPS: {}", self.frames);
            context
                .graphics()
                .set_title(&format!("Cube Example - FPS: {}", self.frames))?;

            self.second_time -= 1.0;
            self.frames = 0;
        }

        Ok(())
    }
}

impl colosseum::InitialScene for CubeScene {
    fn new(
        _: &<Self::Game as colosseum::Game>::Options,
        context: &mut colosseum::UpdateContext<Self::Game>,
    ) -> colosseum::Result<Self> {
        let logger = context.logs().logger("cube");

        colosseum::info!(logger, "Starting main cube scene!");

        let camera = context.graphics().create_camera(
            colosseum::graphics::CameraProjection::Perspective {
                fov: 3.14 / 4.0,
                near: 0.01,
                far: 1000.0,
            },
        )?;

        camera
            .borrow_mut()
            .set_position(colosseum::math::Vector3f::new(0.0, 0.0, -10.0));

        let mesh = context.graphics().create_mesh(VERTICES, INDICES)?;
        let mesh_renderer =
            colosseum::graphics::MeshRenderer::new(context.graphics().default_material(), mesh);

        Ok(CubeScene {
            logger,
            second_time: 0.0,
            frames: 0,
            camera,
            mesh_renderer,
        })
    }
}

const VERTICES: &[colosseum::graphics::Vertex] = &[
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(-1.0, -1.0, -1.0),
        color: colosseum::math::Color3f::new(1.0, 0.0, 0.0),
    },
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(1.0, -1.0, -1.0),
        color: colosseum::math::Color3f::new(0.0, 1.0, 0.0),
    },
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(1.0, 1.0, -1.0),
        color: colosseum::math::Color3f::new(0.0, 0.0, 1.0),
    },
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(-1.0, 1.0, -1.0),
        color: colosseum::math::Color3f::new(1.0, 1.0, 0.0),
    },
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(-1.0, -1.0, 1.0),
        color: colosseum::math::Color3f::new(1.0, 0.0, 1.0),
    },
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(1.0, -1.0, 1.0),
        color: colosseum::math::Color3f::new(0.0, 1.0, 1.0),
    },
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(1.0, 1.0, 1.0),
        color: colosseum::math::Color3f::new(1.0, 1.0, 1.0),
    },
    colosseum::graphics::Vertex {
        position: colosseum::math::Vector3f::new(-1.0, 1.0, 1.0),
        color: colosseum::math::Color3f::new(0.0, 0.0, 0.0),
    },
];

const INDICES: &[u32] = &[
    0, 1, 3, 3, 1, 2, 1, 5, 2, 2, 5, 6, 5, 4, 6, 6, 4, 7, 4, 0, 7, 7, 0, 3, 3, 2, 7, 7, 2, 6, 4, 5,
    0, 0, 5, 1,
];
