use crate::render::{Assets, LoadingMesh, public_data::mesh::loading::LoadingMeshState};

impl LoadingMesh {
    /// Is the mesh transfer complete?
    pub fn is_complete(&mut self, assets: &mut Assets) -> bool {
        match &mut self.inner {
            LoadingMeshState::ReadingFile { op, name } => {
                if !op.as_mut().unwrap().is_complete() {
                    return false;
                }

                match op.take().unwrap().result() {
                    Ok(mesh) => {
                        let (vertices, indices) = mesh.unwrap();
                        match assets.create_mesh(
                            name,
                            vertices.into_iter().map(Into::into).collect(),
                            indices,
                        ) {
                            Ok(transfer) => {
                                *self = transfer;
                                false
                            }
                            Err(error) => {
                                self.inner = LoadingMeshState::Error(error);
                                true
                            }
                        }
                    }
                    Err(error) => {
                        self.inner = LoadingMeshState::Error(error);
                        true
                    }
                }
            }
            LoadingMeshState::Transferring(receiver) => receiver.is_available(),
            LoadingMeshState::Error(_) => true,
        }
    }
}
