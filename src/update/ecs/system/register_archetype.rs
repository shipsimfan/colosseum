use crate::update::System;
use std::any::TypeId;

impl System {
    /// Optionally register an archetype with this system to be used during execution
    pub(in crate::update::ecs) fn register_archetype(
        &mut self,
        index: usize,
        archetype: &[TypeId],
    ) {
        // Temporarily store the archetype index
        self.indices.push(index);

        // Determine if the archetype contains all the components that the system operates on
        let mut matched_components = 0;
        for (i, type_id) in self.component_type_ids.iter().enumerate() {
            for component in archetype {
                if component == type_id {
                    self.indices.push(i);
                    matched_components += 1;
                    break;
                }
            }

            if matched_components == self.component_type_ids.len() {
                break;
            }
        }

        if matched_components == self.component_type_ids.len() {
            // This archetype contains all the components that the system operates on, so we can
            // leave the indices as they
            return;
        }

        // This archetype does not contain all the components that the system operates on, so we
        // need to remove the indices we just added
        self.indices
            .truncate(self.indices.len() - matched_components - 1);
    }
}
