use crate::{FunctionSystem, WorldAccess};
use std::marker::PhantomData;

impl<Input, State, F> FunctionSystem<Input, State, F> {
    /// Create a new [`FunctionSystem`]
    pub fn new(f: F, access: WorldAccess) -> FunctionSystem<Input, State, F> {
        FunctionSystem {
            f,
            state: None,
            access,
            _marker: PhantomData,
        }
    }
}
