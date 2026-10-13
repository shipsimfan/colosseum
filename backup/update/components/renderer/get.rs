use crate::{render::MaterialId, update::components::Renderer};

impl Renderer {
    /// Get the ID of the material used to render the object
    pub fn material(&self) -> MaterialId {
        self.material
    }
}
