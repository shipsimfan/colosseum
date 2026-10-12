use crate::resources::Resources;

impl Resources {
    /// Create a new, empty set of [`Resources`]
    pub(crate) fn new() -> Resources {
        Resources {
            resources: Vec::new(),
        }
    }
}
