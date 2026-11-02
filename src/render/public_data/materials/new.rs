use crate::render::{GraphicsDevice, Materials, UnlitMaterial};
use alexandria::{SlotMap, math::Color4f};
use std::sync::Arc;

impl Materials {
    /// Create a new set of [`Materials`]
    pub(in crate::render::public_data) fn new(device: &Arc<GraphicsDevice>) -> Materials {
        let mut unlit_materials = SlotMap::new();

        let default_unlit_material = unlit_materials.insert(UnlitMaterial::new(
            Color4f::WHITE,
            todo!("add default unlit shader"),
        ));

        Materials {
            unlit_materials,
            default_unlit_material,
            device: device.clone(),
        }
    }
}
