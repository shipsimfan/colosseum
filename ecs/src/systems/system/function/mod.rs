use std::marker::PhantomData;

mod into_system;
mod system;

/// A system that wraps a function and its input type
pub struct FunctionSystem<Input, F> {
    /// The function to be executed by this system
    f: F,

    /// A marker for the input type of the function
    _marker: PhantomData<fn() -> Input>,
}
