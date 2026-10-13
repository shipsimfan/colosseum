use crate::update::GpuAllocatedMemory;

/// The confirmation that a render object has been removed, and the memory can be freed
pub enum RenderObjectRemoveConfirm {
    /// A mesh has been removed from the render job, and the memory is no longer needed
    Mesh(GpuAllocatedMemory),
}
