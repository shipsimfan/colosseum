use crate::{render::GpuRenderable, update::Transform};

impl GpuRenderable {
    /// Create a new [`GpuRenderable`]
    pub(in crate::render::public_data::renderer) fn new(transform: &Transform) -> GpuRenderable {
        GpuRenderable {
            model: transform.matrix(),
        }
    }
}
