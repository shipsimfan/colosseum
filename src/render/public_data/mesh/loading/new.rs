use crate::{
    SingleValueReceiver,
    render::{LoadingMesh, Mesh},
};

impl LoadingMesh {
    /// Create a new [`LoadingMesh`]
    pub(in crate::render) fn new(receiver: SingleValueReceiver<Mesh>) -> LoadingMesh {
        LoadingMesh { receiver }
    }
}
