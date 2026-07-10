use crate::{render::MaterialId, update::components::Renderer};

impl Renderer {
    /// Create a new [`Renderer`] component
    pub fn new(material: MaterialId) -> Renderer {
        Renderer { material }
    }
}
