use crate::render::RenderObjects;
use alexandria::SlotMap;

impl RenderObjects {
    /// Create a new set of [`RenderObjects`]
    pub fn new() -> RenderObjects {
        RenderObjects {
            meshes: SlotMap::new(),
            unlit_opaque_materials: SlotMap::new(),
        }
    }
}
