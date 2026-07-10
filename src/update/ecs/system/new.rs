use crate::update::ecs::{Archetype, System};
use std::any::TypeId;

impl<ExtraData> System<ExtraData> {
    /// Create a new [`System`] over `Components`
    pub fn new(
        component_type_ids: &[TypeId],
        system: Box<dyn Fn(&mut [Archetype], &[usize], &mut ExtraData)>,
    ) -> System<ExtraData> {
        System {
            component_type_ids: component_type_ids.to_vec(),
            indices: Vec::new(),
            system,
        }
    }
}
