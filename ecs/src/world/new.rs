use crate::{Archetypes, Resources, Systems, World};

impl World {
    /// Create a new [`World`]
    pub fn new() -> World {
        World {
            resources: Resources::new(),
            archetypes: Archetypes::new(),
            systems: Systems::new(),
        }
    }
}
