use crate::graphics::ManagedGraphicsObjects;

mod new;

/// The objects that are managed by the engine
pub struct ManagedObjects {
    /// The graphics objects which are managed by the engine
    pub graphics: ManagedGraphicsObjects,
}
