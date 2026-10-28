use crate::{QueryState, SystemParamState};
use std::any::TypeId;

impl SystemParamState for QueryState {
    fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {
        todo!("check if the components match the components and filters")
    }
}
