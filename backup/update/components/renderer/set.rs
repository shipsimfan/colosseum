use crate::{render::MaterialId, update::components::Renderer};

impl Renderer {
    /// Set the material used to render the object
    pub fn set_material(&mut self, material: MaterialId) -> &mut Renderer {
        self.material = material;
        self
    }
}
