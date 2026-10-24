use crate::{
    Result,
    render::{Assets, LoadingMesh, Mesh, Vertex},
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanSharingMode};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl Assets {
    /// Create a new mesh
    ///
    /// The mesh cannot be used in rendering until the [`LoadingMesh`] has completed
    pub fn create_mesh(
        &mut self,
        name: &str,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Result<LoadingMesh> {
        // Create the vertex and index buffers
        let mut vertex_buffer = self
            .device
            .create_buffer(
                0,
                vertices.len() as u64 * std::mem::size_of::<Vertex>() as u64,
                VulkanBufferUsageFlag::VertexBuffer | VulkanBufferUsageFlag::TransferDst,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(|error| {
                new_error!(
                    "unable to create a vertex buffer for \"{}\" - {}",
                    name,
                    error
                )
            })?;
        #[cfg(debug_assertions)]
        self.device
            .set_object_name(
                &mut vertex_buffer,
                &CString::new(format!("\"{}\" Vertex Buffer", name)).unwrap(),
            )
            .map_err(|error| {
                new_error!("unable to name the \"{}\" vertex buffer - {}", name, error)
            })?;

        let mut index_buffer = self
            .device
            .create_buffer(
                0,
                indices.len() as u64 * std::mem::size_of::<u32>() as u64,
                VulkanBufferUsageFlag::IndexBuffer | VulkanBufferUsageFlag::TransferDst,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(|error| {
                new_error!(
                    "unable to create an index buffer for \"{}\" - {}",
                    name,
                    error
                )
            })?;
        #[cfg(debug_assertions)]
        self.device
            .set_object_name(
                &mut index_buffer,
                &CString::new(format!("\"{}\" Index Buffer", name)).unwrap(),
            )
            .map_err(|error| {
                new_error!("unable to name the \"{}\" index buffer - {}", name, error)
            })?;

        // Allocate memory for the vertex and index buffers
        let vertex_memory_requirements = vertex_buffer.get_memory_requirements();
        let index_memory_requirements = index_buffer.get_memory_requirements();
        let memory_requirements = vertex_memory_requirements.extend(&index_memory_requirements);
        let index_buffer_offset = vertex_memory_requirements
            .size()
            .next_multiple_of(index_memory_requirements.alignment())
            as u32;

        let memory = self.mesh_allocator.allocate(&memory_requirements, name)?;
        memory.bind_buffer(&mut vertex_buffer, 0)?;
        memory.bind_buffer(&mut index_buffer, index_buffer_offset)?;

        // Create the mesh
        Mesh::new(
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            memory,
            &self.gpu_transfer_queue,
        )
    }
}
