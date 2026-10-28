use std::any::TypeId;

/// The state associated with a system parameter
pub trait SystemParamState {
    /// Registers the archetype with the system parameter state
    #[allow(unused_variables)]
    fn register_archetype(&mut self, archetype: usize, components: &[TypeId]) {}
}

impl SystemParamState for () {
    fn register_archetype(&mut self, _: usize, _: &[TypeId]) {}
}
