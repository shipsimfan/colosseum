use crate::render::{MaterialId, MaterialKind, RenderMaterial};

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
}
