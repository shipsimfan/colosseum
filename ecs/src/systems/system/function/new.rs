use crate::FunctionSystem;
use std::marker::PhantomData;

impl<Input, State, F> FunctionSystem<Input, State, F> {
    /// Create a new [`FunctionSystem`]
    pub fn new(f: F) -> FunctionSystem<Input, State, F> {
        FunctionSystem {
            f,
            state: None,
            _marker: PhantomData,
        }
    }
}
