use crate::{
    Error, Result,
    render::{
        Material, MaterialId, MaterialKind, Mesh, MeshTransfer, RenderData, Shader, ShaderCode,
        ShaderId, ShaderKind, Vertex,
    },
    update::UpdateRenderObjects,
};
use alexandria::gpu::{VulkanBufferUsageFlag, VulkanSharingMode};

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
            ShaderKind::Lit => self.lit_shaders.insert(shader),
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
            ShaderKind::Lit => &self.lit_shaders[shader.id()],
        };

        let (material, render_material) = Material::new(
            kind,
            shader,
            self.fixed_render_objects.material_pipeline_layout(kind),
            &self.device,
        )?;

        render_data.add_render_object_change((kind, render_material));

        let id = match kind {
            MaterialKind::UnlitOpaque => self.unlit_opaque_materials.insert(material),
            MaterialKind::LitOpaque => self.lit_opaque_materials.insert(material),
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
    ) -> Result<MeshTransfer> {
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
            .map_err(Error::new_inner)?;
        let mut index_buffer = self
            .device
            .create_buffer(
                0,
                indices.len() as u64 * std::mem::size_of::<u32>() as u64,
                VulkanBufferUsageFlag::IndexBuffer | VulkanBufferUsageFlag::TransferDst,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(Error::new_inner)?;

        // Allocate memory for the vertex and index buffers
        let vertex_memory_requirements = vertex_buffer.get_memory_requirements();
        let index_memory_requirements = index_buffer.get_memory_requirements();
        let memory_requirements = vertex_memory_requirements.extend(&index_memory_requirements);
        let index_buffer_offset = vertex_memory_requirements
            .size()
            .next_multiple_of(index_memory_requirements.alignment())
            as u32;

        let memory = self.mesh_allocator.allocate(&memory_requirements)?;
        memory.bind_buffer(&mut vertex_buffer, 0)?;
        memory.bind_buffer(&mut index_buffer, index_buffer_offset)?;

        // Create the mesh
        Mesh::new(
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            memory,
            &mut self.transfer_queue,
        )
    }
}
