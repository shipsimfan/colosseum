use crate::update::ecs::{Archetype, System};

impl<ExtraData> System<ExtraData> {
    /// Optionally register an archetype with this system to be used during execution
    pub(in crate::update::ecs) fn register_archetype(
        &mut self,
        index: usize,
        archetype: &Archetype,
    ) {
        // Temporarily store the archetype index
        self.indices.push(index);

        // Determine if the archetype contains all the components that the system operates on
        let mut matched_components = 0;
        for &type_id in &self.component_type_ids {
            let mut found = false;
            for (i, component) in archetype.components().iter().enumerate() {
                if component.type_id() == type_id {
                    self.indices.push(i);
                    matched_components += 1;
                    found = true;
                    break;
                }
            }

            if !found {
                // This archetype does not contain all the components that the system operates on, so we
                // need to remove the indices we just added
                self.indices
                    .truncate(self.indices.len() - matched_components - 1);
                return;
            }

            if matched_components == self.component_type_ids.len() {
                // This archetype contains all the components that the system operates on, so we can
                // add it to the list of archetypes for this system
                return;
            }
        }
    }
}
