use crate::{
    ReadMeshFile, SingleValueReceiver,
    render::{LoadingMesh, Mesh, public_data::mesh::loading::LoadingMeshState},
};

impl LoadingMesh {
    /// Create a new [`LoadingMesh`] in the reading state
    pub(in crate::render) fn new_reading(op: ReadMeshFile, name: String) -> LoadingMesh {
        LoadingMesh {
            inner: LoadingMeshState::ReadingFile { op: Some(op), name },
        }
    }

    /// Create a new [`LoadingMesh`] in the transferring state
    pub(in crate::render) fn new_transfer(receiver: SingleValueReceiver<Mesh>) -> LoadingMesh {
        LoadingMesh {
            inner: LoadingMeshState::Transferring(receiver),
        }
    }
}
