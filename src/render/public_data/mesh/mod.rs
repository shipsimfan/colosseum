use crate::render::GpuAllocatedMemory;
use alexandria::gpu::VulkanBuffer;

mod loading;
mod vertex;

mod bind;
mod get;
mod new;

pub use loading::*;
pub use vertex::*;

/// A mesh is a collection of vertices and indices that define a 3D object
pub struct Mesh {
    /// The vertices of the mesh
    vertices: Vec<Vertex>,

    /// The indices of the mesh
    indices: Vec<u32>,

    /// The buffer for the vertices of the mesh
    vertex_buffer: VulkanBuffer,

    /// The buffer for the indices of the mesh
    index_buffer: VulkanBuffer,

    /// The device memory containing the buffers
    memory: GpuAllocatedMemory,
}
