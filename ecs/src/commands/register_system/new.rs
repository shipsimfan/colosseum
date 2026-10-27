use crate::{IntoSystem, PhaseName, RegisterSystem};
use std::marker::PhantomData;

impl<Phase: PhaseName, Input: 'static, System: 'static + IntoSystem<Input>>
    RegisterSystem<Phase, Input, System>
{
    /// Create a new [`RegisterSystem`] command
    pub fn new(phase: Phase, system: System) -> RegisterSystem<Phase, Input, System> {
        RegisterSystem {
            phase,
            system,
            _marker: PhantomData,
        }
    }
}
