use crate::{Transforms, util::Arena};

impl Transforms {
    /// Create a new empty set of [`Transforms`]
    pub(in crate::managed_objects) fn new() -> Self {
        Transforms {
            arena: Arena::new(),
        }
    }
}
