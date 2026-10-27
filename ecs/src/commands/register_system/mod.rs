use crate::{Commands, IntoSystem, PhaseName};
use std::marker::PhantomData;

mod command;
mod new;

/// Register a new system in the ECS framework
pub struct RegisterSystem<Phase: PhaseName, Input: 'static, System: 'static + IntoSystem<Input>> {
    /// The phase to register the system in
    phase: Phase,

    /// The system to register
    system: System,

    /// A marker for the input type of the system
    _marker: PhantomData<fn() -> Input>,
}

impl Commands {
    /// Register a new system in the specified phase
    pub fn register_system<
        Phase: PhaseName,
        Input: 'static,
        System: 'static + IntoSystem<Input>,
    >(
        &mut self,
        phase: Phase,
        system: System,
    ) {
        self.queue(RegisterSystem::new(phase, system));
    }
}
