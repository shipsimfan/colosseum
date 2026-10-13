use crate::{
    render::{MaterialId, MaterialKind, Mesh, RenderMaterial, RenderMesh},
    update::GpuAllocatedMemory,
};
use alexandria::{
    Id,
    math::{Color4f, Linear},
};

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

    /// A material has changed color
    ChangeMaterialColor {
        /// The material that was changed
        material: MaterialId,

        /// The new color of the material
        color: Color4f<Linear>,
    },

    /// A material has changed specular strength
    ChangeSpecularStrength {
        /// The material that was changed
        material: MaterialId,

        /// The new specular strength of the material
        specular_strength: f32,
    },

    /// A material has changed shininess
    ChangeShininess {
        /// The material that was changed
        material: MaterialId,

        /// The new shininess of the material
        shininess: f32,
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
