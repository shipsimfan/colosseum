use crate::{
    render::{Material, MaterialId, MaterialKind, Mesh, ShaderId},
    update::UpdateRenderObjects,
};
use alexandria::Id;

impl UpdateRenderObjects {
    /// Get the ID of the default unlit shader
    pub fn default_unlit_shader(&self) -> ShaderId {
        self.default_unlit_shader
    }

    /// Get a reference to the material with the given ID
    pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
        match id.kind() {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.get(id.id()),
        }
    }

    /// Get a mutable reference to the material with the given ID
    pub fn get_material_mut(&mut self, id: MaterialId) -> Option<&mut Material> {
        match id.kind() {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.get_mut(id.id()),
        }
    }

    /// Get the quad primitive
    pub fn quad(&self) -> Id<Mesh> {
        self.quad
    }

    /// Get the plane primitive
    pub fn plane(&self) -> Id<Mesh> {
        self.plane
    }

    /// Get the cube primitive
    pub fn cube(&self) -> Id<Mesh> {
        self.cube
    }

    /// Get the sphere primitive
    pub fn sphere(&self) -> Id<Mesh> {
        self.sphere
    }

    /// Get the cylinder primitive
    pub fn cylinder(&self) -> Id<Mesh> {
        self.cylinder
    }
}
