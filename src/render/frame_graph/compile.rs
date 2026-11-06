use crate::{
    Result,
    render::{
        AntiAliasing, DEPTH_FORMAT, DescriptorSetLayouts, DirectionalLight, FrameGraph,
        GpuDataBuffers, HDR_FORMAT, PointLight, Samplers, Skybox, SpotLight,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind, FrameGraphResources,
            FrameGraphStructure, FxaaNode, LitForwardRenderNode, ProceduralSkyNode,
            QuantizationNode, RenderScaleNode, ShadowMapNode, SkyboxType, SolidColorSkyNode,
            ToneMapNode, UnlitForwardRenderNode,
        },
    },
};
use alexandria::gpu::VulkanAttachmentLoadOp;
use colosseum_ecs::Commands;

impl<Game: crate::Game> FrameGraph<Game> {
    /// Compile the frame graph, preparing it for execution
    pub(in crate::render::frame_graph) fn compile(
        &mut self,
        resources: &mut FrameGraphResources,
        commands: &mut Commands,
        data_buffers: &GpuDataBuffers,
        descriptor_set_layouts: &DescriptorSetLayouts,
        samplers: &Samplers,
        settings: &Game::SettingsCache,
        skybox: &Skybox,
    ) -> Result<()> {
        let structure = FrameGraphStructure::new::<Game>(settings, skybox);
        if self.structure.as_ref() == Some(&structure) {
            return Ok(()); // No need to recompile if the structure hasn't changed
        }

        let mut builder =
            FrameGraphBuilder::new(commands, &self.device, data_buffers, descriptor_set_layouts);

        /* Shadow Map Passes */
        ShadowMapNode::<DirectionalLight>::add(&mut builder)?;
        ShadowMapNode::<PointLight>::add(&mut builder)?;
        ShadowMapNode::<SpotLight>::add(&mut builder)?;

        /* Main Render Passes */
        let main_color_output = builder.create_resource(
            "Main Color Output",
            HDR_FORMAT,
            FrameGraphResourceKind::Render,
            VulkanAttachmentLoadOp::DontCare,
        );
        let depth_buffer = builder.create_resource(
            "Main Depth Buffer",
            DEPTH_FORMAT,
            FrameGraphResourceKind::Render,
            VulkanAttachmentLoadOp::Clear,
        );

        match structure.skybox() {
            SkyboxType::SolidColor => SolidColorSkyNode::add(main_color_output, &mut builder)?,
            SkyboxType::Procedural => ProceduralSkyNode::add(main_color_output, &mut builder)?,
        }
        UnlitForwardRenderNode::add(main_color_output, depth_buffer, &mut builder)?;
        LitForwardRenderNode::add(main_color_output, depth_buffer, samplers, &mut builder)?;

        /* Post Processing */

        // Perform tone mapping
        let tone_map_output = ToneMapNode::<Game>::add(main_color_output, samplers, &mut builder)?;

        // Perform render scaling, if needed
        let scaled_output = if structure.render_scale() {
            RenderScaleNode::add(tone_map_output, &mut builder)?
        } else {
            tone_map_output
        };

        // Perform anti-aliasing
        let aa_output = match structure.anti_aliasing() {
            AntiAliasing::None => scaled_output,
            AntiAliasing::FXAA => FxaaNode::add(scaled_output, samplers, &mut builder)?,
        };

        // Quantization
        QuantizationNode::add(
            aa_output,
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            samplers,
            &mut builder,
        )?;

        // Finish the frame graph compilation
        builder.finish(resources, descriptor_set_layouts)?;
        self.structure = Some(structure);
        Ok(())
    }
}
