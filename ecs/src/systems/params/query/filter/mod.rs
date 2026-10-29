use std::any::TypeId;

mod and;
mod or;
mod with;
mod without;

pub use and::*;
pub use or::*;
pub use with::*;
pub use without::*;

/// An item that can be used as a filter in a query
pub trait QueryFilter {
    /// Determine if an archetype matches the filter
    fn filter(components: &[TypeId]) -> bool;
}
