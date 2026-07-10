use crate::{
    Result,
    render::{
        Material, MaterialId, MaterialKind, RenderData, Shader, ShaderCode, ShaderId, ShaderKind,
    },
    update::UpdateRenderObjects,
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
}
