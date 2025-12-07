use crate::{Transform, util::Arena};

mod create;
mod get;
mod index;
mod iter;
mod new;
mod remove;

/// A set of [`Transform`]s registered with the engine
pub struct Transforms {
    /// The set of [`Transform`]s
    arena: Arena<Transform>,
}
