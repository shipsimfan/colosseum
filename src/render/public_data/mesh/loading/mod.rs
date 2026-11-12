use crate::{ReadMeshFile, SingleValueReceiver, render::Mesh};
use colosseum_core::Error;

mod is_complete;
mod new;
mod take;
mod wait;

/// A mesh being loaded
pub struct LoadingMesh {
    /// The current state of the transfer
    inner: LoadingMeshState,
}

/// The state of a mesh being loaded
enum LoadingMeshState {
    /// The mesh is being read from the file
    ReadingFile {
        /// The read mesh file operation
        op: Option<ReadMeshFile>,

        /// The name of the mesh
        name: String,
    },

    /// The mesh is being transferred to the GPU
    Transferring(SingleValueReceiver<Mesh>),

    /// Error
    Error(Error),
}
