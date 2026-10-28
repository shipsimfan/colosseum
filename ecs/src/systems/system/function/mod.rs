use crate::WorldAccess;
use std::marker::PhantomData;

mod new;
mod system;

/// A system that wraps a function and its input type
pub struct FunctionSystem<Input, State, F> {
    /// The function to be executed by this system
    f: F,

    /// The state associated with this system
    state: Option<State>,

    /// The access information for this system
    access: WorldAccess,

    /// A marker for the input type of the function
    _marker: PhantomData<fn() -> Input>,
}
