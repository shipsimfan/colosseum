use crate::render::{MaterialId, Mesh, Renderer};
use alexandria::Id;

impl Renderer {
    /// Create a new [`Renderer`] component
    pub fn new<M: Into<MaterialId>>(material: M, mesh: Id<Mesh>) -> Renderer {
        Renderer {
            material: material.into(),
            mesh,
        }
    }
}
