use crate::systems::Phase;

impl Phase {
    /// Create a new [`Phase`]
    pub fn new() -> Phase {
        Phase {
            systems: Vec::new(),
        }
    }
}
