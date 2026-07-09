use crate::update::ecs::{Archetype, System};
use std::any::TypeId;

impl System {
    /// Create a new [`System`] over `Components`
    #[allow(private_bounds)]
    #[allow(private_interfaces)]
    pub(in crate::update::ecs) fn new(
        component_type_ids: &[TypeId],
        system: Box<dyn Fn(&mut [Archetype], &[usize])>,
    ) -> System {
        System {
            component_type_ids: component_type_ids.to_vec(),
            indices: Vec::new(),
            system,
        }
    }
}
