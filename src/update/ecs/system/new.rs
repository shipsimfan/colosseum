use crate::update::{System, ecs::Archetype};
use std::any::TypeId;

impl System {
    /// Create a new [`System`] over `Components`
    #[allow(private_bounds)]
    #[allow(private_interfaces)]
    pub fn new(
        component_type_ids: &[TypeId],
        system: Box<dyn FnMut(&mut [Archetype], &[usize])>,
    ) -> System {
        System {
            component_type_ids: component_type_ids.to_vec(),
            indices: Vec::new(),
            system,
        }
    }
}
