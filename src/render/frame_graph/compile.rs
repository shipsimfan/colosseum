use crate::{
    Result, SettingsCache,
    render::{
        DescriptorSetLayouts, FrameGraph, FrameIndex, GraphicsDevice, HDR_FORMAT, SDR_FORMAT,
        Samplers, Swapchain,
        frame_graph::{
            FrameGraphBuilder, FrameGraphResourceId, FrameGraphResourceKind, FrameGraphResources,
            QuantizationNode, SolidColorSkyNode, ToneMapNode,
        },
        phases::FrameGraphSetup,
    },
};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanImageAspectFlag};
use colosseum_ecs::{Commands, Res, ResMut, World};

impl<Game: crate::Game> FrameGraph<Game> {
    pub fn register_prepare(world: &mut World) {
        world.register_system(FrameGraphSetup, FrameGraph::<Game>::prepare);
    }

    /// Prepare the frame graph for execution
    fn prepare(
        mut self: ResMut<Self>,
        mut resources: ResMut<FrameGraphResources>,
        commands: &mut Commands,
        settings: Res<Game::SettingsCache>,
        device: Res<GraphicsDevice>,
        swapchain: Res<Swapchain>,
        frame_index: Res<FrameIndex>,
        descriptor_set_layouts: Res<DescriptorSetLayouts>,
        samplers: Res<Samplers>,
    ) -> Result<()> {
        if !self.compiled {
            self.compile(
                &mut resources,
                commands,
                &device,
                &descriptor_set_layouts,
                &samplers,
            )?;
        }

        // Handle resizing if needed
        resources.resize(
            swapchain.size(),
            settings.display_settings().render_scale(),
            &frame_index,
            &device,
        )?;

        // Update swapchain image in frame graph resources
        resources.insert_external(
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            swapchain.image_view(),
            swapchain.size(),
            1,
            VulkanImageAspectFlag::Color,
        );

        Ok(())
    }

    /// Compile the frame graph, preparing it for execution
    fn compile(
        &mut self,
        resources: &mut FrameGraphResources,
        commands: &mut Commands,
        device: &GraphicsDevice,
        descriptor_set_layouts: &DescriptorSetLayouts,
        samplers: &Samplers,
    ) -> Result<()> {
        let mut builder = FrameGraphBuilder::new(commands, device);

        /* Main Render Passes */
        let main_color_output = builder.create_resource(
            "Main Color Output",
            HDR_FORMAT,
            FrameGraphResourceKind::Native,
            VulkanAttachmentLoadOp::DontCare,
        );

        SolidColorSkyNode::add(main_color_output, &mut builder)?;

        /* Post Processing */

        // Perform tone mapping
        let tone_map_output = builder.create_resource(
            "Tone Map Output",
            SDR_FORMAT,
            FrameGraphResourceKind::Native,
            VulkanAttachmentLoadOp::DontCare,
        );
        ToneMapNode::<Game>::add(main_color_output, tone_map_output, samplers, &mut builder)?;

        /*
        // Perform render scaling, if needed
        let scaled_output = if structure.has_render_scale() {
            let scale_output = resources.create_native_scale_transient("Scaled Output", SDR_FORMAT);
            nodes.push(RenderScaleNode::new(tone_map_output, scale_output).into());
            scale_output
        } else {
            tone_map_output
        };

        // Perform anti-aliasing
        let aa_output = match structure.anti_aliasing() {
            AntiAliasingMode::None => scaled_output,
            AntiAliasingMode::FXAA => {
                let aa_output = resources.create_native_scale_transient("FXAA Output", SDR_FORMAT);
                nodes.push(FxaaNode::new(scaled_output, aa_output).into());
                aa_output
            }
        };
        */

        // Quantization
        QuantizationNode::add(
            tone_map_output,
            FrameGraphResourceId::SWAPCHAIN_IMAGE,
            samplers,
            &mut builder,
        )?;

        builder.finish(resources, descriptor_set_layouts)?;

        self.compiled = true;
        Ok(())
    }
}
