use crate::{
    Result,
    render::{
        AntiAliasing, DescriptorSetLayouts, FrameGraph, HDR_FORMAT, Samplers,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind, FrameGraphResources,
            FrameGraphStructure, FxaaNode, QuantizationNode, RenderScaleNode, SolidColorSkyNode,
            ToneMapNode,
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
        descriptor_set_layouts: &DescriptorSetLayouts,
        samplers: &Samplers,
        settings: &Game::SettingsCache,
    ) -> Result<()> {
        let structure = FrameGraphStructure::new::<Game>(settings);
        if self.structure.as_ref() == Some(&structure) {
            return Ok(()); // No need to recompile if the structure hasn't changed
        }

        let mut builder = FrameGraphBuilder::new(commands, &self.device);

        /* Main Render Passes */
        let main_color_output = builder.create_resource(
            "Main Color Output",
            HDR_FORMAT,
            FrameGraphResourceKind::Render,
            VulkanAttachmentLoadOp::DontCare,
        );

        SolidColorSkyNode::add(main_color_output, &mut builder)?;

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
