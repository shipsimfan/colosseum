use crate::update::ecs::{Archetype, System};

impl<ExtraData> System<ExtraData> {
    /// Execute the system with the given archetypes
    pub(in crate::update::ecs) fn execute(
        &self,
        archetypes: &mut [Archetype],
        extra_data: &mut ExtraData,
    ) {
        (self.system)(archetypes, &self.indices, extra_data);
    }
}
