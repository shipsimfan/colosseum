use crate::render::{
    FixedRenderObjects, RenderData, RenderObjects, as_bytes,
    frame_graph::{
        FrameGraphResources, ShadowMapLight, ShadowMapNode, nodes::shadow_map::PushConstants,
    },
};
use alexandria::{
    gpu::{
        VulkanAttachmentLoadOp, VulkanAttachmentStoreOp, VulkanCommandBuffer, VulkanImageLayout,
        VulkanPipelineBindPoint, VulkanRenderingAttachmentInfo, VulkanResolveModeFlag,
        VulkanShaderStageFlag, VulkanViewport,
    },
    math::{Color4f, Linear, Recti, Vector2i},
};

impl<L: ShadowMapLight> ShadowMapNode<L> {
    /// Execute the lit forward pass, rendering all lit objects in the scene
    pub(in crate::render::frame_graph::nodes) fn execute(
        &self,
        render_data: &RenderData,
        render_objects: &RenderObjects,
        resources: &FrameGraphResources,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        let shadow_map = resources.shadow_map_buffer(L::SHADOW_MAP);
        let size = shadow_map.size();

        let lights = L::get_lights(render_data.lighting());

        for i in 0..lights.count() {
            // Begin the render pass for this light
            cmd_buffer.cmd_begin_rendering(
                0,
                Vector2i::ZERO,
                size,
                L::CASCADES as _,
                0,
                &[],
                Some(&VulkanRenderingAttachmentInfo::new(
                    shadow_map.image_view(i),
                    VulkanImageLayout::DepthAttachmentOptimal,
                    VulkanResolveModeFlag::None,
                    None,
                    VulkanImageLayout::Undefined,
                    VulkanAttachmentLoadOp::Clear,
                    VulkanAttachmentStoreOp::Store,
                    Color4f::<Linear>::new(1.0, 1.0, 1.0, 1.0),
                )),
                None,
            );

            // Bind the viewport and scissor for the render pass
            let viewport = VulkanViewport::new(0.0, 0.0, size.x as _, size.y as _, 0.0, 1.0);
            let scissor = Recti::new(Vector2i::ZERO, size);
            cmd_buffer.cmd_set_viewport(0, &[viewport]);
            cmd_buffer.cmd_set_scissor(0, &[scissor]);

            // Bind the pipeline
            let pipeline = render_objects.pipeline(FixedRenderObjects::SHADOW_MAP_PIPELINE);
            pipeline.bind(cmd_buffer);

            let pipeline_layout = pipeline.layout();

            // Bind the descriptor sets
            cmd_buffer.cmd_bind_descriptor_set(
                VulkanPipelineBindPoint::Graphics,
                pipeline_layout,
                0,
                resources.descriptor_set(L::DESCRIPTOR_SET),
            );

            for (_, mesh, object_data) in render_data.lit_opaque_renderables() {
                let mesh = render_objects.mesh(mesh);

                mesh.bind(cmd_buffer);

                let push_constants = PushConstants {
                    light_index: (i * L::CASCADES) as _,
                    object_index: object_data as _,
                };
                cmd_buffer.cmd_push_constants(
                    pipeline_layout,
                    VulkanShaderStageFlag::Vertex,
                    0,
                    unsafe { as_bytes(&push_constants) },
                );

                cmd_buffer.cmd_draw_indexed(mesh.index_count(), L::CASCADES as _, 0, 0, 0);
            }

            cmd_buffer.cmd_end_rendering();
        }
    }
}
