use crate::{
    render::{MaterialId, MaterialKind, Mesh, RenderMaterial, RenderMesh},
    update::GpuAllocatedMemory,
};
use alexandria::Id;

mod apply;
mod from;

/// The changes to the set of registered materials
pub(crate) enum RenderObjectChange {
    /** Material Changes **/

    /// A new material has been added
    AddMaterial {
        /// The kind of material that was added
        kind: MaterialKind,

        /// The material that was added
        material: RenderMaterial,
    },

    /// A material has been removed
    RemoveMaterial {
        /// The material that was removed
        material: MaterialId,
    },

    /** Mesh Changes **/

    /// A new mesh has been added
    AddMesh {
        /// The mesh that was added
        mesh: RenderMesh,
    },

    /// A mesh has been removed
    RemoveMesh {
        /// The mesh that was removed
        mesh: Id<Mesh>,

        /// The allocated memory that was used by the mesh, which can be freed
        memory: GpuAllocatedMemory,
    },
}
