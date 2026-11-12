use crate::{
    Result,
    render::{LoadingMesh, Mesh, public_data::mesh::loading::LoadingMeshState},
};

impl LoadingMesh {
    /// Take the result of the mesh transfer
    pub(in crate::render::public_data) fn take(self) -> Result<Mesh> {
        match self.inner {
            LoadingMeshState::Transferring(receiver) => Ok(receiver.take()),
            LoadingMeshState::ReadingFile { .. } => {
                panic!("cannot take mesh while still reading file")
            }
            LoadingMeshState::Error(error) => Err(error),
        }
    }
}
