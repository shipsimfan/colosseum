#![feature(const_trait_impl)]

colosseum::run!(Cube);

const CUBE_VERTICES: &[colosseum::render::Vertex] = &[
    // Front (+Z)
    colosseum::render::Vertex::new((-1.0, 1.0, 1.0), (0.0, 0.0, 0.0), (0.0, 0.0, 1.0)),
    colosseum::render::Vertex::new((1.0, 1.0, 1.0), (0.0, 0.0, 1.0), (0.0, 0.0, 1.0)),
    colosseum::render::Vertex::new((-1.0, -1.0, 1.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)),
    colosseum::render::Vertex::new((1.0, -1.0, 1.0), (0.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
    // Back (-Z)
    colosseum::render::Vertex::new((-1.0, 1.0, -1.0), (1.0, 0.0, 0.0), (0.0, 0.0, -1.0)),
    colosseum::render::Vertex::new((1.0, 1.0, -1.0), (1.0, 0.0, 1.0), (0.0, 0.0, -1.0)),
    colosseum::render::Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0, 0.0), (0.0, 0.0, -1.0)),
    colosseum::render::Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    // Top (+Y)
    colosseum::render::Vertex::new((-1.0, 1.0, 1.0), (0.0, 0.0, 0.0), (0.0, 1.0, 0.0)),
    colosseum::render::Vertex::new((-1.0, 1.0, -1.0), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)),
    colosseum::render::Vertex::new((1.0, 1.0, 1.0), (0.0, 0.0, 1.0), (0.0, 1.0, 0.0)),
    colosseum::render::Vertex::new((1.0, 1.0, -1.0), (1.0, 0.0, 1.0), (0.0, 1.0, 0.0)),
    // Bottom (-Y)
    colosseum::render::Vertex::new((-1.0, -1.0, 1.0), (0.0, 1.0, 0.0), (0.0, -1.0, 0.0)),
    colosseum::render::Vertex::new((1.0, -1.0, 1.0), (0.0, 1.0, 1.0), (0.0, -1.0, 0.0)),
    colosseum::render::Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0, 0.0), (0.0, -1.0, 0.0)),
    colosseum::render::Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (0.0, -1.0, 0.0)),
    // Left (-X)
    colosseum::render::Vertex::new((-1.0, 1.0, 1.0), (0.0, 0.0, 0.0), (-1.0, 0.0, 0.0)),
    colosseum::render::Vertex::new((-1.0, -1.0, 1.0), (0.0, 1.0, 0.0), (-1.0, 0.0, 0.0)),
    colosseum::render::Vertex::new((-1.0, 1.0, -1.0), (1.0, 0.0, 0.0), (-1.0, 0.0, 0.0)),
    colosseum::render::Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0, 0.0), (-1.0, 0.0, 0.0)),
    // Right (+X)
    colosseum::render::Vertex::new((1.0, 1.0, 1.0), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0)),
    colosseum::render::Vertex::new((1.0, 1.0, -1.0), (1.0, 0.0, 1.0), (1.0, 0.0, 0.0)),
    colosseum::render::Vertex::new((1.0, -1.0, 1.0), (0.0, 1.0, 1.0), (1.0, 0.0, 0.0)),
    colosseum::render::Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (1.0, 0.0, 0.0)),
];
const CUBE_INDICES: &[u32] = &[
    0, 1, 2, 1, 3, 2, // Front
    4, 6, 5, 5, 6, 7, // Back
    8, 9, 10, 10, 9, 11, // Top
    12, 13, 14, 13, 15, 14, // Bottom
    16, 17, 18, 17, 19, 18, // Left
    20, 21, 22, 22, 21, 23, // Right
];

/// The cube example
struct Cube;

impl colosseum::Game for Cube {
    type Options = CubeOptions;
    type SettingsCache = CubeSettings;
    type InitialScene = CubeInitialScene;

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
struct CubeInitialScene {
    /// The mesh being transferred to the GPU
    mesh: Option<colosseum::render::MeshTransfer>,
}

impl colosseum::update::Scene for CubeInitialScene {
    type Game = Cube;

    fn update(
        &mut self,
        context: &mut colosseum::update::UpdateContext<Self::Game>,
    ) -> colosseum::Result<()> {
        if !self.mesh.as_ref().unwrap().is_complete() {
            return Ok(());
        }

        let mesh = context.complete_mesh(self.mesh.take().unwrap());
        context.set_next_scene(move |context| CubeMainScene::new(context, mesh));
        Ok(())
    }
}

impl colosseum::update::InitialScene for CubeInitialScene {
    fn new(
        _: &CubeOptions,
        context: &mut colosseum::update::UpdateContext<Cube>,
    ) -> colosseum::Result<Self> {
        let mesh = Some(context.create_mesh(CUBE_VERTICES.to_vec(), CUBE_INDICES.to_vec())?);

        Ok(CubeInitialScene { mesh })
    }
}

/// The main scene for the cube example
struct CubeMainScene {
    /// A count of the number of frames that have been rendered
    frames: usize,

    /// The amount of time that has passed since the last display of FPS
    fps_timer: f32,

    /// The logger for the cube example
    logger: colosseum::logging::Logger,

    /// The material used to render the cube
    material: colosseum::render::MaterialId,

    /// The mesh used to render the cube
    mesh: colosseum::Id<colosseum::render::Mesh>,

    /// The ID of the renderable cube
    cube: colosseum::Id<colosseum::update::Entity>,

    /// The ID of the camera
    camera: colosseum::Id<colosseum::update::Entity>,

    /// Should the cube be rendered?
    render_state: bool,
}

impl colosseum::update::Scene for CubeMainScene {
    type Game = Cube;

    fn update(
        &mut self,
        context: &mut colosseum::update::UpdateContext<Cube>,
    ) -> colosseum::Result<()> {
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
                let rotation = transform.rotation() * euler;

                transform.set_rotation(rotation);
            } else {
                change = transform.rotation().rotate(change);
                *transform.position_mut() += change;
            }
        }

        // Render the cube
        if context.inputs().key_down(colosseum::Key::V) {
            self.render_state = !self.render_state;
            if self.render_state {
                context.ecs_mut().add_component(
                    self.cube,
                    colosseum::update::components::Renderer::new(self.material, self.mesh),
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

impl CubeMainScene {
    /// Create a new [`CubeMainScene`]
    fn new(
        context: &mut colosseum::update::UpdateContext<Cube>,
        mesh: colosseum::Id<colosseum::render::Mesh>,
    ) -> colosseum::Result<Self> {
        // Create logger
        let logger = context.logger("cube");

        // Create material for the cube
        let shader = context.default_lit_shader();
        let material =
            context.create_material(colosseum::render::MaterialKind::LitOpaque, shader)?;
        context.set_material_specular_strength(material, 1.0);
        context.set_material_shininess(material, 256.0);

        // Create the cube entity and add components
        let ecs = context.ecs_mut();
        let cube = ecs.create_entity();
        let mut transform = colosseum::update::components::Transform::default();
        transform.set_position((0.0, 0.0, 3.0));
        ecs.add_component(cube, transform);
        ecs.add_component(
            cube,
            colosseum::update::components::Renderer::new(material, mesh),
        );

        // Create a directional light
        let directional_light = ecs.create_entity();
        ecs.add_component(
            directional_light,
            colosseum::update::components::DirectionalLight::new(
                (1.0, 1.0, 1.0),
                0.5,
                (-1.0, -1.5, 1.0),
            ),
        );

        // Create a point light
        let point_light = ecs.create_entity();
        ecs.add_component(
            point_light,
            colosseum::update::components::PointLight::new(
                (1.0, 1.0, 1.0),
                1.0,
                (-1.2, -1.2, 4.2),
                2.0,
            ),
        );

        // Create a spot light
        let spot_light = ecs.create_entity();
        ecs.add_component(
            spot_light,
            colosseum::update::components::SpotLight::new(
                (1.0, 0.95, 0.85),
                1.0,
                (0.0, 0.0, 6.0),
                5.0,
                (0.0, 0.0, -1.0),
                3.14 / 8.0,
                3.14 / 12.0,
            ),
        );

        // Create the camera entity and add components
        let camera = ecs.create_entity();
        ecs.add_component(camera, colosseum::update::components::Camera::default());
        ecs.add_component(camera, colosseum::update::components::Transform::default());
        context.set_active_camera(camera);

        // Set skybox
        context.set_skybox(colosseum::update::ProceduralSkybox::new_light(
            (0.5, 0.7, 0.9),
            directional_light,
            0.02,
            5.0,
            1.0,
            (0.37, 0.29, 0.20),
        ));

        Ok(CubeMainScene {
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
