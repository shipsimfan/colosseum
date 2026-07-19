use std::sync::Arc;

use crate::{
    Error, Result,
    render::{
        Material, MaterialId, MaterialKind, Mesh, MeshTransfer, RenderData, Shader, ShaderCode,
        ShaderId, ShaderKind, Vertex,
    },
    update::UpdateRenderObjects,
};
use alexandria::{
    Id,
    gpu::{
        VulkanBuffer, VulkanBufferUsageFlag, VulkanDevice, VulkanDeviceMemory, VulkanSharingMode,
    },
};

impl UpdateRenderObjects {
    /// Create a new [`Shader`]
    pub fn create_shader<const N: usize>(
        &mut self,
        kind: ShaderKind,
        code: &ShaderCode<N>,
    ) -> Result<ShaderId> {
        let shader = Shader::new(code, &self.device)?;
        let id = match kind {
            ShaderKind::Unlit => self.unlit_shaders.insert(shader),
        };
        Ok(ShaderId::new(kind, id))
    }

    /// Create a new [`Material`]
    pub fn create_material(
        &mut self,
        kind: MaterialKind,
        shader: ShaderId,
        render_data: &mut RenderData,
    ) -> Result<MaterialId> {
        debug_assert!(shader.kind().is_compatible_with(kind), "
            The shader must be compatible with the material kind. The shader kind is {:?} and the material kind is {:?}",
            shader.kind(),
            kind,
        );

        let shader = match shader.kind() {
            ShaderKind::Unlit => &self.unlit_shaders[shader.id()],
        };

        let (material, render_material) = Material::new(
            shader,
            &self.pipeline_layout,
            self.swapchain_format,
            &self.device,
        )?;

        render_data.add_render_object_change((kind, render_material));

        let id = match kind {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.insert(material),
        };
        Ok(MaterialId::new(kind, id))
    }

    /// Create a new mesh
    ///
    /// The mesh cannot be used in rendering until the [`MeshTransfer`] has completed
    pub fn create_mesh(
        &mut self,
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Result<(Id<Mesh>, MeshTransfer)> {
        let (vertex_buffer, vertex_memory) = create_buffer(
            &self.device,
            (vertices.len() * std::mem::size_of::<Vertex>()) as u64,
            VulkanBufferUsageFlag::VertexBuffer,
            self.device_local_memory_type,
        )?;
        let (index_buffer, index_memory) = create_buffer(
            &self.device,
            (indices.len() * std::mem::size_of::<u32>()) as u64,
            VulkanBufferUsageFlag::IndexBuffer,
            self.device_local_memory_type,
        )?;

        let (mesh, transfer) = Mesh::new(
            vertices,
            indices,
            vertex_buffer,
            vertex_memory.clone(),
            index_buffer,
            index_memory.clone(),
            &mut self.transfer_queue,
        )?;
        let id = self.meshes.insert((mesh, vertex_memory, index_memory));
        Ok((unsafe { id.cast() }, transfer))
    }
}

/// Create a new buffer and allocate memory for it
fn create_buffer(
    device: &VulkanDevice,
    size: u64,
    usage: VulkanBufferUsageFlag,
    memory_type: usize,
) -> Result<(VulkanBuffer, Arc<VulkanDeviceMemory>)> {
    let mut buffer = device
        .create_buffer(
            0,
            size,
            usage | VulkanBufferUsageFlag::TransferDst,
            VulkanSharingMode::Exclusive,
            &[],
        )
        .map_err(Error::new_inner)?;

    let memory_requirements = buffer.get_memory_requirements();
    let memory = device
        .allocate_memory(memory_requirements.size(), memory_type)
        .map_err(Error::new_inner)?;

    buffer.bind_memory(&memory, 0).map_err(Error::new_inner)?;

    Ok((buffer, Arc::new(memory)))
}
