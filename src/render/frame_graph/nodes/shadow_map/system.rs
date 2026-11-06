use crate::render::{
    Assets, FrameIndex, HostDataBuffer, LitForwardMaterial, Pipeline, Renderables,
    SecondaryCommandBuffer, ShadowMapLight,
    frame_graph::{FrameGraphResources, ShadowMapNode, nodes::shadow_map::PushConstants},
};
use alexandria::{
    gpu::{
        VulkanAttachmentLoadOp, VulkanAttachmentStoreOp, VulkanImageLayout,
        VulkanPipelineBindPoint, VulkanRenderingAttachmentInfo, VulkanResolveModeFlag,
        VulkanShaderStageFlag, VulkanViewport,
    },
    math::{Color4f, Linear, Recti, Vector2, Vector2i},
};
use colosseum_core::as_bytes;
use colosseum_ecs::{Res, ResMut};

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Executes the shadow map rendering system for the given light type
    pub(in crate::render::frame_graph::nodes::shadow_map) fn system(
        self: Res<Self>,
        lights: Res<HostDataBuffer<L::Gpu>>,
        objects: Res<Renderables<LitForwardMaterial>>,
        assets: Res<Assets>,
        resources: Res<FrameGraphResources>,
        pipeline: Res<Pipeline<L>>,
        frame_index: Res<FrameIndex>,
        mut cmd_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        let cmd_buffer = &mut cmd_buffer[&frame_index];
        let shadow_map = resources.get_shadow_map(L::SHADOW_MAP);
        let pipeline_layout = pipeline.layout();
        let size = shadow_map.size(&frame_index);

        for light_index in 0..lights[&frame_index].count() {
            // Start a render pass
            cmd_buffer.cmd_begin_rendering(
                0,
                Vector2i::ZERO,
                size,
                L::CASCADES,
                0,
                &[],
                Some(&VulkanRenderingAttachmentInfo::new(
                    shadow_map.image_view(&frame_index, light_index),
                    VulkanImageLayout::DepthAttachmentOptimal,
                    VulkanResolveModeFlag::None,
                    None,
                    VulkanImageLayout::Undefined,
                    VulkanAttachmentLoadOp::Clear,
                    VulkanAttachmentStoreOp::Store,
                    Color4f::<Linear>::new(0.0, 0.0, 0.0, 0.0),
                )),
                None,
            );

            // Bind the viewport and scissor
            let viewport = VulkanViewport::new(0.0, 0.0, size.x as _, size.y as _, 0.0, 1.0);
            let scissor = Recti::new(Vector2::ZERO, size);

            cmd_buffer.cmd_set_viewport(0, &[viewport]);
            cmd_buffer.cmd_set_scissor(0, &[scissor]);

            // Bind the pipeline
            pipeline.bind(cmd_buffer);

            // Bind the descriptor sets
            cmd_buffer.cmd_bind_descriptor_set(
                VulkanPipelineBindPoint::Graphics,
                pipeline_layout,
                0,
                resources.descriptor_set(self.descriptor_set, &frame_index),
            );

            // Render each lit object
            for (_, mesh, object_index) in objects.iter() {
                // Bind the push constants
                let push_constants = PushConstants {
                    light_index: light_index as _,
                    object_index: object_index as _,
                };
                cmd_buffer.cmd_push_constants(
                    pipeline_layout,
                    VulkanShaderStageFlag::Vertex | VulkanShaderStageFlag::Fragment,
                    0,
                    as_bytes(&push_constants),
                );

                // Render the mesh
                assets.mesh(mesh).draw(cmd_buffer, L::CASCADES);
            }

            // End the render pass
            cmd_buffer.cmd_end_rendering();
        }
    }
}
