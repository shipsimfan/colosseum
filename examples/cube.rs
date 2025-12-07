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
    camera: colosseum::graphics::CameraHandle,
}

impl colosseum::InitialScene for CubeScene {
    fn new(
        _: &<Self::Game as colosseum::Game>::Options,
        context: &mut colosseum::UpdateContext<Self::Game>,
    ) -> colosseum::Result<Self> {
        let logger = context.logs.logger("cube");

        colosseum::info!(logger, "Starting main cube scene!");

        let camera =
            context
                .graphics
                .create_camera(colosseum::graphics::CameraProjection::Perspective {
                    fov: 3.14 / 4.0,
                    near: 0.01,
                    far: 1000.0,
                })?;

        context
            .graphics
            .camera_mut(camera)
            .set_position(colosseum::math::Vector3f::new(0.0, 0.0, -10.0));

        let mesh = colosseum::graphics::MeshPrimitives::cube();
        let material = context.graphics.default_lit_material();

        let mesh_renderer = context.graphics.create_mesh_renderer(material, mesh, 1)?;
        let mesh_renderer = context.graphics.mesh_renderer_mut(mesh_renderer);
        mesh_renderer.push();

        let mut transform = colosseum::math::Transform::new();
        transform.set_scale(colosseum::math::Vector3::new(2.0, 2.0, 2.0));
        mesh_renderer.update(0, &mut transform);

        context.graphics.create_directional_light(
            colosseum::math::Vector3f::new(-1.0, -2.0, 3.0),
            colosseum::math::Color3f::new(1.0, 0.95, 0.85),
            1.0,
        );

        context.graphics.create_point_light(
            colosseum::math::Vector3::new(-2.0, -3.0, 4.0),
            15.0,
            colosseum::math::Color3::new(0.2, 0.2, 0.8),
            1.0,
        );

        Ok(CubeScene {
            logger,
            second_time: 0.0,
            frames: 0,
            camera,
        })
    }
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
                .graphics
                .set_title(&format!("Cube Example - FPS: {}", self.frames))?;

            self.second_time -= 1.0;
            self.frames = 0;
        }

        let delta_t = context.delta_t();
        let camera = context.graphics.camera_mut(self.camera);
        if context.input.key(colosseum::input::KeyCode::LeftShift)
            || context.input.key(colosseum::input::KeyCode::RightShift)
        {
            let mut rotation = camera.rotation();
            if context.input.key(colosseum::input::KeyCode::W)
                || context.input.key(colosseum::input::KeyCode::UpArrow)
            {
                rotation = colosseum::math::Quaternion::angle_axis(
                    delta_t,
                    colosseum::math::Vector3f::UNIT_X,
                ) * rotation;
            }
            if context.input.key(colosseum::input::KeyCode::S)
                || context.input.key(colosseum::input::KeyCode::DownArrow)
            {
                rotation = colosseum::math::Quaternion::angle_axis(
                    -delta_t,
                    colosseum::math::Vector3f::UNIT_X,
                ) * rotation;
            }
            if context.input.key(colosseum::input::KeyCode::A)
                || context.input.key(colosseum::input::KeyCode::LeftArrow)
            {
                rotation = colosseum::math::Quaternion::angle_axis(
                    -delta_t,
                    colosseum::math::Vector3f::UNIT_Y,
                ) * rotation;
            }
            if context.input.key(colosseum::input::KeyCode::D)
                || context.input.key(colosseum::input::KeyCode::RightArrow)
            {
                rotation = colosseum::math::Quaternion::angle_axis(
                    delta_t,
                    colosseum::math::Vector3f::UNIT_Y,
                ) * rotation;
            }
            if context.input.key(colosseum::input::KeyCode::Q) {
                rotation = colosseum::math::Quaternion::angle_axis(
                    delta_t,
                    colosseum::math::Vector3f::UNIT_Z,
                ) * rotation;
            }
            if context.input.key(colosseum::input::KeyCode::E) {
                rotation = colosseum::math::Quaternion::angle_axis(
                    -delta_t,
                    colosseum::math::Vector3f::UNIT_Z,
                ) * rotation;
            }

            camera.set_rotation(rotation);
        } else {
            let camera_rotation = camera.rotation();
            let right = camera_rotation.right();
            let up = camera_rotation.up();
            let forward = camera_rotation.forward();

            let mut translation = colosseum::math::Vector3f::ZERO;
            if context.input.key(colosseum::input::KeyCode::W)
                || context.input.key(colosseum::input::KeyCode::UpArrow)
            {
                translation += forward * delta_t;
            }
            if context.input.key(colosseum::input::KeyCode::S)
                || context.input.key(colosseum::input::KeyCode::DownArrow)
            {
                translation -= forward * delta_t;
            }
            if context.input.key(colosseum::input::KeyCode::A)
                || context.input.key(colosseum::input::KeyCode::LeftArrow)
            {
                translation -= right * delta_t;
            }
            if context.input.key(colosseum::input::KeyCode::D)
                || context.input.key(colosseum::input::KeyCode::RightArrow)
            {
                translation += right * delta_t;
            }
            if context.input.key(colosseum::input::KeyCode::Q) {
                translation += up * delta_t;
            }
            if context.input.key(colosseum::input::KeyCode::E) {
                translation -= up * delta_t;
            }

            if translation != colosseum::math::Vector3f::ZERO {
                let position = camera.position();
                camera.set_position(position + translation);
            }
        }

        Ok(())
    }
}
