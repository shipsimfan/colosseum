use crate::render::{LoadingMesh, Mesh};

impl LoadingMesh {
    /// Take the result of the mesh transfer
    pub(in crate::render::public_data) fn take(self) -> Mesh {
        self.receiver.take()
    }
}
