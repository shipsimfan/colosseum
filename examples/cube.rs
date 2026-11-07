#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]

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

colosseum::run!(
    Cube,
    |world: &mut colosseum::ecs::World| -> colosseum::Result<()> {
        let mut assets = world.get_resource_mut::<colosseum::render::Assets>();
        let mesh_loading = MeshLoading::new(&mut assets)?;
        world.insert_resource(mesh_loading);

        world.register_system(colosseum::update::Update, MeshLoading::system);

        Ok(())
    }
);

/// The cube example
struct Cube {
    /// The ID of the mesh associated with this cube
    mesh: colosseum::Id<colosseum::render::Mesh>,

    /// The ID of the material associated with this cube
    material: colosseum::Id<colosseum::render::LitForwardMaterial>,
}

impl colosseum::GameMetadata for Cube {
    const NAME: &str = "Cube Example";
    const COMPANY: &str = "Lance Hart";
    const VERSION: &str = env!("CARGO_PKG_VERSION");
}

impl colosseum::Game for Cube {
    type Options = CubeOptions;
    type SettingsCache = CubeSettings;
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
#[colosseum::settings_cache]
struct CubeSettings {}

/// The resource for handling mesh loading
struct MeshLoading {
    /// The mesh transfer handle for the cube mesh
    transfer: Option<colosseum::render::LoadingMesh>,
}

impl MeshLoading {
    fn new(assets: &mut colosseum::render::Assets) -> colosseum::Result<MeshLoading> {
        let transfer = assets.create_mesh("Cube", CUBE_VERTICES.to_vec(), CUBE_INDICES.to_vec())?;

        Ok(MeshLoading {
            transfer: Some(transfer),
        })
    }

    fn system(
        mut self: colosseum::ecs::ResMut<MeshLoading>,
        mut next_scene: colosseum::ecs::ResMut<colosseum::update::NextScene>,
        mut assets: colosseum::ecs::ResMut<colosseum::render::Assets>,
    ) {
        match &mut self.transfer {
            Some(transfer) => {
                if !transfer.is_complete() {
                    return;
                }
            }
            None => return,
        };

        let mesh = assets.complete_mesh(self.transfer.take().unwrap());
        next_scene.set(move |world: &mut colosseum::ecs::World| init_main_scene(world, mesh));
    }
}

/// Initialize the main scene
fn init_main_scene(
    world: &mut colosseum::ecs::World,
    cube_mesh: colosseum::Id<colosseum::render::Mesh>,
) -> colosseum::Result<()> {
    // Add FPS counter
    world.insert_resource(FpsCounter::new());
    world.register_system(colosseum::update::Update, FpsCounter::tick);

    // Add input handling system
    world.register_system(colosseum::update::Update, input_handling);

    // Get materials
    let mut materials = world.get_resource_mut::<colosseum::render::Materials>();
    let cube_material = materials.create_lit_forward((1.0, 1.0, 1.0), 1.0, 256.0, None);
    let sphere_material = materials.default_lit_forward();

    // Get the sphere mesh
    let sphere_mesh = world
        .get_resource::<colosseum::render::Primitives>()
        .sphere();

    // Spawn the cube entity with its renderer and transform
    let cube = world.spawn((
        colosseum::update::Transform::new_at((0.0, 0.0, 3.0)),
        Cube {
            mesh: cube_mesh,
            material: cube_material,
        },
        colosseum::render::Renderer::new(cube_material, cube_mesh),
    ));
    colosseum::debug_s!("cube", "Cube ID: {}", cube);

    // Create a sphere
    world.spawn((
        colosseum::update::Transform::new(
            (0.6, 0.0, 5.0),
            colosseum::math::Quaternionf::IDENTITY,
            0.5,
        ),
        colosseum::render::Renderer::new(sphere_material, sphere_mesh),
    ));

    // Create a directional light
    let directional_light = world.spawn((colosseum::render::DirectionalLight::new(
        (1.0, 1.0, 1.0),
        0.5,
        (-1.0, -1.5, 1.0),
        10.0,
        0.97,
    ),));

    // Create a point light
    world.spawn((colosseum::render::PointLight::new(
        (1.0, 1.0, 1.0),
        1.0,
        (-1.2, -1.2, 4.2),
        5.0,
    ),));

    // Create a spot light
    world.spawn((
        colosseum::render::SpotLight::new(
            (1.0, 0.95, 0.85),
            1.0,
            (0.0, 0.0, 0.0),
            10.0,
            (0.0, 0.0, 1.0),
            3.14 / 8.0,
            3.14 / 12.0,
        ),
        SpotLightState {
            angle: std::f32::consts::PI / 2.0,
        },
    ));

    // Set skybox
    world.insert_resource(colosseum::render::Skybox::Procedural(
        colosseum::render::ProceduralSkybox::new_light(
            (0.5, 0.7, 0.9),
            directional_light,
            0.003,
            5.0,
            1.0,
            (0.37, 0.29, 0.20),
        ),
    ));

    Ok(())
}

/// A resource for counting frames per second (FPS)
struct FpsCounter {
    /// The current number of frames in the last second
    frames: u32,

    /// The time at which the last second started
    last_time: std::time::Instant,

    /// The logger for reporting FPS information
    logger: colosseum::Logger,
}

impl FpsCounter {
    /// Create a new [`FpsCounter`]
    pub fn new() -> FpsCounter {
        FpsCounter {
            frames: 0,
            last_time: std::time::Instant::now(),
            logger: colosseum::logger!("cube"),
        }
    }

    /// Tick the FPS counter
    pub fn tick(mut self: colosseum::ecs::ResMut<FpsCounter>) {
        self.frames += 1;

        let now = std::time::Instant::now();
        if now.duration_since(self.last_time).as_secs() >= 1 {
            colosseum::info!(self.logger, "FPS: {}", self.frames);
            self.frames = 0;
            self.last_time = now;
        }
    }
}

/// The state associated with the spot light
struct SpotLightState {
    /// The angle of the spot light
    angle: f32,
}

fn input_handling(
    input: colosseum::ecs::Res<colosseum::update::Input>,
    settings: colosseum::ecs::Res<CubeSettings>,
    window: colosseum::ecs::Res<colosseum::Window>,
    delta_time: colosseum::ecs::Res<colosseum::update::DeltaTime>,
    mut cube: colosseum::ecs::Single<(
        &colosseum::Id<colosseum::ecs::Entity>,
        &mut colosseum::update::Transform,
        Option<&colosseum::render::Renderer>,
        &Cube,
    )>,
    mut camera: colosseum::ecs::ResMut<colosseum::render::Camera>,
    mut spot_light: colosseum::ecs::Single<(
        &mut colosseum::render::SpotLight,
        &mut SpotLightState,
    )>,
    commands: &mut colosseum::ecs::Commands,
) -> colosseum::Result<()> {
    // Toggle fullscreen mode when the user presses F11
    if input.key_down(colosseum::Key::F11) {
        if settings.display().fullscreen() {
            window.unset_fullscreen()?;
        } else {
            window.set_fullscreen()?;
        }
    }

    // Toggle visibility of the cube when the user presses V
    if input.key_down(colosseum::Key::V) {
        if cube.2.is_some() {
            colosseum::debug_s!("cube", "Cube ID: {}", cube.0);
            commands.remove_component::<colosseum::render::Renderer>(*cube.0);
        } else {
            commands.add_component::<colosseum::render::Renderer>(
                *cube.0,
                colosseum::render::Renderer::new(cube.3.material, cube.3.mesh),
            );
        }
    }

    // Move the camera based on user input
    let rotating = input.key(colosseum::Key::LeftShift) | input.key(colosseum::Key::RightShift);
    let changing_cube =
        input.key(colosseum::Key::LeftControl) | input.key(colosseum::Key::RightControl);

    let mut change = colosseum::math::Vector3f::ZERO;
    let mut changed = false;
    let speed = delta_time.as_secs_f32();
    if rotating {
        if input.key(colosseum::Key::W) {
            change.x += speed;
            changed = true;
        }
        if input.key(colosseum::Key::S) {
            change.x -= speed;
            changed = true;
        }
        if input.key(colosseum::Key::A) {
            change.y -= speed;
            changed = true;
        }
        if input.key(colosseum::Key::D) {
            change.y += speed;
            changed = true;
        }
        if input.key(colosseum::Key::Q) {
            change.z += speed;
            changed = true;
        }
        if input.key(colosseum::Key::E) {
            change.z -= speed;
            changed = true;
        }
    } else {
        if input.key(colosseum::Key::W) {
            change.z += speed;
            changed = true;
        }
        if input.key(colosseum::Key::S) {
            change.z -= speed;
            changed = true;
        }
        if input.key(colosseum::Key::A) {
            change.x -= speed;
            changed = true;
        }
        if input.key(colosseum::Key::D) {
            change.x += speed;
            changed = true;
        }
        if input.key(colosseum::Key::Q) {
            change.y += speed;
            changed = true;
        }
        if input.key(colosseum::Key::E) {
            change.y -= speed;
            changed = true;
        }
    }

    if changed {
        if rotating {
            let change = colosseum::math::Quaternionf::from_euler_angles(change);
            if changing_cube {
                cube.1.rotation *= change;
            } else {
                camera.rotation *= change;
            }
        } else {
            if changing_cube {
                change = cube.1.rotation.rotate(change);
                cube.1.position += change;
            } else {
                change = camera.rotation.rotate(change);
                camera.position += change;
            }
        }
    }

    // Update the spot light angle based on user input
    if input.key(colosseum::Key::OpenBracket) {
        spot_light.1.angle -= speed;
    }
    if input.key(colosseum::Key::CloseBracket) {
        spot_light.1.angle += speed;
    }

    spot_light.1.angle = spot_light.1.angle % (2.0 * std::f32::consts::PI);

    let position =
        colosseum::math::Vector3f::new(spot_light.1.angle.cos(), 0.0, spot_light.1.angle.sin())
            * 5.0
            + colosseum::math::Vector3f::new(0.0, 0.0, 3.0);

    let direction = colosseum::math::Vector3f::new(0.0, 0.0, 3.0) - position;

    spot_light.0.position = position;
    spot_light.0.direction = direction.normalized();

    Ok(())
}
