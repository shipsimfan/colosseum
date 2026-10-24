use crate::{SingleValueReceiver, render::Mesh};

mod is_complete;
mod new;
mod take;
mod wait;

/// The state of a mesh transfer
pub struct LoadingMesh {
    /// The receiver for a single value
    receiver: SingleValueReceiver<Mesh>,
}
