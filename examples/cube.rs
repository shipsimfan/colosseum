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
        let logger = context.logger("cube");

        let shader = context.create_shader(&TRIANGLE_SHADER)?;
        let material = context.create_material(shader)?;

        let mut entities = Vec::new();
        for i in 1usize..=10 {
            let entity = context.ecs_mut().create_entity();
            colosseum::debug!(logger, "Created entity {}: {}", i, entity);

            assert_eq!(
                *context
                    .ecs()
                    .get::<colosseum::Id<colosseum::update::Entity>>(entity),
                entity
            );

            if i % 2 == 0 {
                context.ecs_mut().add_component(entity, i);
                colosseum::debug!(logger, "Added integer to entity {}", entity);
            } else {
                context.ecs_mut().add_component(entity, i as f32);
                colosseum::debug!(logger, "Added float to entity {}", entity);
            }

            if i % 3 == 0 {
                context.ecs_mut().add_component(entity, i.to_string());
                colosseum::debug!(logger, "Added string to entity {}", entity);
            }

            entities.push(entity);
        }

        let last_entity = entities.pop().unwrap();
        context.ecs_mut().remove_entity(last_entity);
        assert!(
            context
                .ecs()
                .try_get::<colosseum::Id<colosseum::update::Entity>>(last_entity)
                .is_none()
        );
        colosseum::debug!(
            logger,
            "Removed entity {}: {}",
            entities.len() + 1,
            last_entity
        );

        for i in 0..entities.len() / 2 {
            context.ecs_mut().remove_entity(entities[i]);
            colosseum::debug!(logger, "Removed entity {}: {}", i + 1, entities[i]);
        }

        colosseum::debug!(
            logger,
            "Remaining entities: {}",
            context.ecs().num_entities()
        );

        let mut remove_string = None;
        for entity in context.ecs().entities() {
            colosseum::debug!(logger, "Remaining entity: {} ", entity);

            if let Some(i) = context.ecs().try_get::<usize>(entity) {
                colosseum::debug!(logger, "Entity {} has integer: {}", entity, i);
            }
            if let Some(f) = context.ecs().try_get::<f32>(entity) {
                colosseum::debug!(logger, "Entity {} has float: {}", entity, f);
            }
            if let Some(s) = context.ecs().try_get::<String>(entity) {
                colosseum::debug!(logger, "Entity {} has string: {}", entity, s);
                remove_string = Some(entity);
            }
        }

        let remove_string =
            remove_string.expect("There should be at least one entity with a string component");
        context.ecs_mut().remove_component::<String>(remove_string);
        assert!(
            context.ecs().try_get::<String>(remove_string).is_none(),
            "The string component should have been removed from the entity"
        );
        colosseum::debug!(
            logger,
            "Removed string component from entity {}",
            remove_string
        );

        let entity_a = context.ecs_mut().create_entity();
        context.ecs_mut().add_component(entity_a, 43usize);
        context.ecs_mut().add_component(entity_a, 65f32);

        let entity_b = context.ecs_mut().create_entity();
        context.ecs_mut().add_component(entity_b, 78f32);
        context.ecs_mut().add_component(entity_b, 12usize);

        type Components<'a> = (&'a [colosseum::Id<colosseum::update::Entity>], &'a [f32]);
        const TYPE_IDS: [std::any::TypeId; 2] = [
            std::any::TypeId::of::<colosseum::Id<colosseum::update::Entity>>(),
            std::any::TypeId::of::<f32>(),
        ];
        const TYPE_COUNT: usize = TYPE_IDS.len();

        let system_logger = logger.clone();
        let system = context
            .ecs_mut()
            .register_ad_hoc_system(colosseum::update::System::new(Box::new(
                move |archetypes: &mut [Archetype], indices: &[usize]| {
                    let archetype_count = indices.len() / (TYPE_COUNT + 1);
                    for i in 0..archetype_count {
                        let archetype_index = indices[i * (TYPE_COUNT + 1)];
                        let component_indices =
                            &indices[i * (TYPE_COUNT + 1) + 1..(i + 1) * (TYPE_COUNT + 1)];

                        let archetype = &mut archetypes[archetype_index];
                        let component_set = (
                            archetype.get_all_at(indices[0]),
                            archetype.get_all_at(indices[1]),
                        );

                        (|(i, f): Components| {
                            for index in 0..i.len() {
                                colosseum::debug!(
                                    system_logger,
                                    "Ad-hoc system: Entity {} has float {}",
                                    i[index],
                                    f[index]
                                );
                            }
                        })(component_set);
                    }
                },
            )));

        Ok(CubeScene {
            color: colosseum::math::ColorHsv::RED,
            frames: 0,
            fps_timer: 0.0,
            logger,
            material,
            render_state: true,
        })
    }
}
