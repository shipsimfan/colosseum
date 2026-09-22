use argparse::Command;
use colosseum::{
    ColosseumOptions, Game, GameOptions, Id, Key, Result, info,
    logging::Logger,
    render::{MaterialId, MaterialKind},
    run,
    update::{
        Entity, InitialScene, ProceduralSkybox, Scene, UpdateContext,
        components::{Camera, DirectionalLight, Renderer, Rigidbody, Transform},
    },
};
use colosseum_macros::settings_cache;
use std::collections::VecDeque;

const MAX_OBJECTS: usize = 16;

run!(PhysicsPlayground);

/// The physics playground game structure
struct PhysicsPlayground;

impl Game for PhysicsPlayground {
    type Options = PhysicsPlaygroundOptions;
    type SettingsCache = PhysicsPlaygroundSettings;
    type InitialScene = PhysicsPlaygroundScene;

    const NAME: &str = "Physics Playground";
    const COMPANY: &str = "Lance Hart";
    const VERSION: &str = env!("CARGO_PKG_VERSION");
}

/// The command line options to control the physics playground game
#[derive(Command)]
#[command(
    help,
    version,
    description = "A physics playground game to experiment with physics simulations"
)]
struct PhysicsPlaygroundOptions {
    /// The options for controlling colosseum
    #[flag_group]
    colosseum_options: ColosseumOptions<PhysicsPlayground>,
}

impl GameOptions<PhysicsPlayground> for PhysicsPlaygroundOptions {
    fn colosseum_options(&self) -> &ColosseumOptions<PhysicsPlayground> {
        &self.colosseum_options
    }
}

/// The settings cache for the physics playground game
#[settings_cache]
struct PhysicsPlaygroundSettings {}

struct PhysicsPlaygroundScene {
    /// The logger for the scene
    logger: Logger,

    /// The camera for the scene
    camera: Id<Entity>,

    /// The material for the objects
    material: MaterialId,

    /// The current velocity to launch objects with
    velocity: usize,

    /// The current objects in the scene
    objects: VecDeque<Id<Entity>>,
}

impl InitialScene for PhysicsPlaygroundScene {
    fn new(_: &PhysicsPlaygroundOptions, context: &mut UpdateContext<Self::Game>) -> Result<Self> {
        let ecs = context.ecs_mut();

        // Create a directional light
        let directional_light = ecs.create_entity();
        ecs.add_component(
            directional_light,
            DirectionalLight::new((1.0, 1.0, 1.0), 0.5, (-1.0, -1.5, 1.0), 10.0, 0.97),
        );

        // Create camera
        let camera = ecs.create_entity();
        ecs.add_component(camera, Camera::default());
        ecs.add_component(camera, Transform::default());
        context.set_active_camera(camera);

        // Create material
        let shader = context.default_lit_shader();
        let material =
            context.create_material("Sphere Material", MaterialKind::LitOpaque, shader)?;

        // Set skybox
        context.set_skybox(ProceduralSkybox::new_light(
            (0.5, 0.7, 0.9),
            directional_light,
            0.003,
            5.0,
            1.0,
            (0.37, 0.29, 0.20),
        ));

        Ok(PhysicsPlaygroundScene {
            logger: context.logger("playground"),
            camera,
            material,
            velocity: 2,
            objects: VecDeque::new(),
        })
    }
}

impl Scene for PhysicsPlaygroundScene {
    type Game = PhysicsPlayground;

    fn update(&mut self, context: &mut UpdateContext<Self::Game>) -> Result<()> {
        // Move the camera based on user input
        move_camera(self.camera, context);

        // Adjust launch velocity
        if context.inputs().key_down(Key::Equals) {
            self.velocity = self.velocity * 2;
            if self.velocity == 0 {
                self.velocity = 1;
            }

            info!(self.logger, "Launch velocity adjusted to {}", self.velocity);
        }

        if context.inputs().key_down(Key::Minus) {
            self.velocity = self.velocity / 2;
            info!(self.logger, "Launch velocity adjusted to {}", self.velocity);
        }

        // Launch a new object if requested
        if context.inputs().key_down(Key::Space) {
            if self.objects.len() == MAX_OBJECTS {
                self.objects.pop_front();
            }

            let sphere = context.sphere();
            let ecs = context.ecs_mut();

            let mut camera_transform = ecs.get::<Transform>(self.camera).clone();
            let forward = camera_transform.rotation().forward();
            *camera_transform.position_mut() += forward * 1.1;

            let entity = ecs.create_entity();
            ecs.add_component(entity, camera_transform);
            ecs.add_component(entity, Renderer::new(self.material, sphere));

            let mut rigidbody = Rigidbody::default();
            rigidbody.set_velocity(forward * self.velocity as f32);
            ecs.add_component(entity, rigidbody);

            self.objects.push_back(entity);
        }

        Ok(())
    }
}

fn move_camera(camera: Id<Entity>, context: &mut UpdateContext<PhysicsPlayground>) {
    // Move the camera based on user input
    let rotating = context.inputs().key(colosseum::Key::LeftShift)
        | context.inputs().key(colosseum::Key::RightShift);

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
            .get_mut::<colosseum::update::components::Transform>(camera);

        if rotating {
            let euler = colosseum::math::Quaternionf::from_euler_angles(change);
            let rotation = transform.rotation() * euler;

            transform.set_rotation(rotation);
        } else {
            change = transform.rotation().rotate(change);
            *transform.position_mut() += change;
        }
    }
}
