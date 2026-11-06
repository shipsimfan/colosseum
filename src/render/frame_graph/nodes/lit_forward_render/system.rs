use crate::render::{
    Assets, FrameIndex, LitForwardMaterial, Materials, Renderables, SecondaryCommandBuffer,
    frame_graph::{FrameGraphResources, LitForwardRenderNode},
};
use alexandria::gpu::VulkanPipelineBindPoint;
use colosseum_ecs::{Res, ResMut};

impl LitForwardRenderNode {
    /// Renders lit objects using the forward rendering pipeline
    pub(in crate::render::frame_graph::nodes::lit_forward_render) fn system(
        self: Res<Self>,
        materials: Res<Materials>,
        assets: Res<Assets>,
        resources: Res<FrameGraphResources>,
        renderables: Res<Renderables<LitForwardMaterial>>,
        frame_index: Res<FrameIndex>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        if renderables.is_empty() {
            return;
        }

        let cmd_buffer = &mut command_buffer[&frame_index];

        // Bind the descriptor sets
        let pipeline_layout = materials.lit_forward_pipeline_layout();
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            0,
            resources.descriptor_set(self.camera_descriptor_set, &frame_index),
        );
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            1,
            resources.descriptor_set(self.renderable_descriptor_set, &frame_index),
        );
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            2,
            resources.descriptor_set(self.light_descriptor_set, &frame_index),
        );

        // Render each renderable object
        for (material, mesh, object_index) in renderables.iter() {
            materials.lit_forward_material(material).bind(
                cmd_buffer,
                pipeline_layout,
                object_index,
            );

            assets.mesh(mesh).draw(cmd_buffer, 1);
        }
    }
}
