use crate::render::{
    FixedRenderObjects, MaterialKind, RenderData, RenderObjects,
    frame_graph::{FrameGraphResources, LitForwardRenderNode},
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanViewport},
    math::{Recti, Vector2},
};

impl LitForwardRenderNode {
    /// Execute the lit forward pass, rendering all lit objects in the scene
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        render_objects: &RenderObjects,
        resources: &FrameGraphResources,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        let size = resources.get(self.output).size();

        // Bind the viewport and scissor for the render pass
        let viewport = VulkanViewport::new(0.0, 0.0, size.x as _, size.y as _, 0.0, 1.0);
        let scissor = Recti::new(Vector2::ZERO, size);
        cmd_buffer.cmd_set_viewport(0, &[viewport]);
        cmd_buffer.cmd_set_scissor(0, &[scissor]);

        // Bind the descriptor sets
        let pipeline_layout = render_objects.material_pipeline_layout(MaterialKind::LitOpaque);
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            0,
            resources.descriptor_set(FixedRenderObjects::CAMERA_DESCRIPTOR_SET),
        );
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            1,
            resources.descriptor_set(FixedRenderObjects::RENDERABLES_DESCRIPTOR_SET),
        );
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            2,
            resources.descriptor_set(FixedRenderObjects::LIGHTING_DESCRIPTOR_SET),
        );

        for (material, mesh, object_data) in render_data.lit_opaque_renderables() {
            let material = render_objects.lit_opaque_material(material);
            let mesh = render_objects.mesh(mesh);

            material.bind(cmd_buffer, pipeline_layout, object_data);
            mesh.bind(cmd_buffer);

            cmd_buffer.cmd_draw_indexed(mesh.index_count(), 1, 0, 0, 0);
        }
    }
}
