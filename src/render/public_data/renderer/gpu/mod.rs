use alexandria::math::Matrix4x4f;

mod new;

/// The GPU representation of a renderable object
#[repr(C)]
#[allow(unused)]
pub(in crate::render) struct GpuRenderable {
    /// The model matrix of the object
    model: Matrix4x4f,
}
