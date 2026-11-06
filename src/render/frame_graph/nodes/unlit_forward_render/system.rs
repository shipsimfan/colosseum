use crate::render::{
    Assets, FrameIndex, Materials, Renderables, SecondaryCommandBuffer, UnlitMaterial,
    frame_graph::{FrameGraphResources, UnlitForwardRenderNode},
};
use alexandria::gpu::VulkanPipelineBindPoint;
use colosseum_ecs::{Res, ResMut};

impl UnlitForwardRenderNode {
    /// Renders unlit objects using the forward rendering pipeline
    pub(in crate::render::frame_graph::nodes::unlit_forward_render) fn system(
        self: Res<Self>,
        materials: Res<Materials>,
        assets: Res<Assets>,
        resources: Res<FrameGraphResources>,
        renderables: Res<Renderables<UnlitMaterial>>,
        frame_index: Res<FrameIndex>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        if renderables.is_empty() {
            return;
        }

        let cmd_buffer = &mut command_buffer[&frame_index];

        // Bind the descriptor sets
        let pipeline_layout = materials.unlit_pipeline_layout();
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

        // Render each renderable object
        for (material, mesh, object_index) in renderables.iter() {
            materials
                .unlit_material(material)
                .bind(cmd_buffer, pipeline_layout, object_index);

            assets.mesh(mesh).draw(cmd_buffer, 1);
        }
    }
}
