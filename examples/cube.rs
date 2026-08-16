#![feature(const_trait_impl)]

colosseum::run!(Cube);

colosseum::render::compile_shader!(
    /// A shader which renders a single triangle
    pub const TRIANGLE_SHADER = "triangle.slang",
    vert_main,
    frag_main
);

const VERTICES: &[colosseum::render::Vertex] = &[
    colosseum::render::Vertex::new(
        colosseum::math::Vector3f::new(0.0, 0.5, 0.0),
        colosseum::math::Color3f::<colosseum::math::Linear>::new(1.0, 0.0, 0.0),
    ),
    colosseum::render::Vertex::new(
        colosseum::math::Vector3f::new(-0.5, -0.5, 0.0),
        colosseum::math::Color3f::<colosseum::math::Linear>::new(0.0, 0.0, 1.0),
    ),
    colosseum::render::Vertex::new(
        colosseum::math::Vector3f::new(0.5, -0.5, 0.0),
        colosseum::math::Color3f::<colosseum::math::Linear>::new(0.0, 1.0, 0.0),
    ),
];

const INDICES: &[u32] = &[0, 1, 2];

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

enum MeshState {
    Loading(Option<colosseum::render::MeshTransfer>),
    Ready(colosseum::Id<colosseum::render::Mesh>),
}

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
    material: colosseum::render::MaterialId,

    /// The mesh used to render the cube
    mesh: MeshState,

    /// The ID of the renderable cube
    cube: colosseum::Id<colosseum::update::Entity>,

    /// The ID of the camera
    camera: colosseum::Id<colosseum::update::Entity>,

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
            || context.inputs().key(colosseum::Key::K)
        {
            self.color.add_hue(amount)
        } else if context.inputs().key(colosseum::Key::Right)
            || context.inputs().key(colosseum::Key::L)
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

        // Move the camera based on user input
        let rotating = context.inputs().key(colosseum::Key::LeftShift)
            | context.inputs().key(colosseum::Key::RightShift);
        let changing_cube = context.inputs().key(colosseum::Key::LeftControl)
            | context.inputs().key(colosseum::Key::RightControl);

        let mut change = colosseum::math::Vector3f::ZERO;
        let mut changed = false;
        let speed = context.delta_time().as_secs_f32();
        if rotating {
            if context.inputs().key(colosseum::Key::W) {
                change.x += speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::S) {
                change.x -= speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::A) {
                change.y -= speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::D) {
                change.y += speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::Q) {
                change.z += speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::E) {
                change.z -= speed;
                changed = true;
            }
        } else {
            if context.inputs().key(colosseum::Key::W) {
                change.z += speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::S) {
                change.z -= speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::A) {
                change.x -= speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::D) {
                change.x += speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::Q) {
                change.y += speed;
                changed = true;
            }
            if context.inputs().key(colosseum::Key::E) {
                change.y -= speed;
                changed = true;
            }
        }

        if changed {
            let transform = context
                .ecs_mut()
                .get_mut::<colosseum::update::components::Transform>(if changing_cube {
                    self.cube
                } else {
                    self.camera
                });

            if rotating {
                let euler = colosseum::math::Quaternionf::from_euler_angles(change);
                let rotation = if changing_cube {
                    transform.rotation() * euler
                } else {
                    euler * transform.rotation()
                };

                transform.set_rotation(rotation);
            } else {
                change = transform.rotation().rotate(change);
                *transform.position_mut() += change;
            }
        }

        // Render the cube
        let mesh = match &mut self.mesh {
            MeshState::Loading(transfer) => {
                if !transfer.as_ref().unwrap().is_complete() {
                    return Ok(());
                }

                let mesh = context.complete_mesh(transfer.take().unwrap());
                context.ecs_mut().add_component(
                    self.cube,
                    colosseum::update::components::Renderer::new(self.material, mesh),
                );
                self.mesh = MeshState::Ready(mesh);
                mesh
            }
            &mut MeshState::Ready(mesh) => mesh,
        };

        if context.inputs().key_down(colosseum::Key::V) {
            self.render_state = !self.render_state;
            if self.render_state {
                context.ecs_mut().add_component(
                    self.cube,
                    colosseum::update::components::Renderer::new(self.material, mesh),
                );
            } else {
                context
                    .ecs_mut()
                    .remove_component::<colosseum::update::components::Renderer>(self.cube);
            }
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
        let logger = context.logger("cube");

        let shader =
            context.create_shader(colosseum::render::ShaderKind::Unlit, &TRIANGLE_SHADER)?;
        let material =
            context.create_material(colosseum::render::MaterialKind::UnlitOpaque, shader)?;

        let mesh = MeshState::Loading(Some(
            context.create_mesh(VERTICES.to_vec(), INDICES.to_vec())?,
        ));

        let ecs = context.ecs_mut();
        let cube = ecs.create_entity();
        let mut transform = colosseum::update::components::Transform::default();
        transform.set_position((0.0, 0.0, 3.0));
        ecs.add_component(cube, transform);

        let camera = ecs.create_entity();
        ecs.add_component(camera, colosseum::update::components::Camera::default());
        ecs.add_component(camera, colosseum::update::components::Transform::default());
        context.set_active_camera(camera);

        Ok(CubeScene {
            color: colosseum::math::ColorHsv::RED,
            frames: 0,
            fps_timer: 0.0,
            logger,
            material,
            mesh,
            cube,
            camera,
            render_state: true,
        })
    }
}
