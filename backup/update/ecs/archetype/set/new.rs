use crate::{
    logging::Logger,
    update::{
        Entity,
        components::{DirectionalLight, PointLight, Renderer, Rigidbody, SpotLight},
        ecs::{Archetype, ArchetypeSet, archetype::Components},
    },
};
use alexandria::{Id, PackedMap, SlotMap};

impl ArchetypeSet {
    /// Create a new [`ArchetypeSet`]
    pub fn new(logger: Logger) -> ArchetypeSet {
        let entity_component = Components::new::<Id<Entity>>();
        let entity_archetype = Archetype::new(vec![entity_component], &logger);

        let mut physics_systems = PackedMap::new();
        physics_systems.insert(Rigidbody::system());

        let mut pre_update_systems = PackedMap::new();
        pre_update_systems.insert(Rigidbody::zero_system());

        let rendering_systems = vec![
            Renderer::system(),
            DirectionalLight::system(),
            PointLight::system(),
            SpotLight::system(),
        ];

        ArchetypeSet {
            archetypes: vec![entity_archetype],
            physics_systems,
            pre_update_systems,
            ad_hoc_systems: SlotMap::new(),
            post_update_systems: PackedMap::new(),
            rendering_systems,
            logger,
        }
    }
}
