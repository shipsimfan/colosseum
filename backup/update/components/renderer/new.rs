use crate::{
    render::{MaterialId, Mesh},
    update::components::Renderer,
};
use alexandria::Id;

impl Renderer {
    /// Create a new [`Renderer`] component
    pub fn new(material: MaterialId, mesh: Id<Mesh>) -> Renderer {
        Renderer { material, mesh }
    }
}
