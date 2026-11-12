use crate::{
    Result,
    render::{LoadingMesh, public_data::mesh::loading::LoadingMeshState},
};

impl LoadingMesh {
    /// Wait for the transfer to complete
    pub(in crate::render) fn wait(&self) -> Result<()> {
        match &self.inner {
            LoadingMeshState::Transferring(receiver) => receiver.wait_no_take(),
            LoadingMeshState::Error(_) | LoadingMeshState::ReadingFile { .. } => {
                panic!("cannot wait on a file-based mesh")
            }
        }
    }
}
