use crate::{QueryData, QueryFilter, QueryState, SystemParamState};
use std::any::TypeId;

impl<Data: QueryData, Filter: QueryFilter> SystemParamState for QueryState<Data, Filter> {
    fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {
        if !Filter::filter(components) {
            return;
        }

        if let Some(state) = Data::create_state(components) {
            self.archetypes.push((archetype, state));
        }
    }
}
