use crate::graphics::ManagedGraphicsObjects;

mod transform;

mod new;

pub use transform::{Transform, TransformHandle, Transforms};

/// The objects that are managed by the engine
pub struct ManagedObjects {
    /// The graphics objects which are managed by the engine
    pub graphics: ManagedGraphicsObjects,

    /// The registered [`Transform`]s
    pub transforms: Transforms,
}
