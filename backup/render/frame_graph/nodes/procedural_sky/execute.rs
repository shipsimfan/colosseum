use crate::render::{
    FixedRenderObjects, RenderData, RenderObjects, RenderSkybox, as_bytes,
    frame_graph::{FrameGraphResources, ProceduralSkyNode, nodes::procedural_sky::PushConstants},
};
use alexandria::{
    gpu::{VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanShaderStageFlag, VulkanViewport},
    math::{Recti, Vector2},
};

impl ProceduralSkyNode {
    /// Execute the procedural sky pass
    pub(in crate::render::frame_graph::nodes) fn execute(
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

        // Bind the pipeline
        let pipeline = render_objects.pipeline(FixedRenderObjects::PROCEDURAL_SKY_PIPELINE);
        pipeline.bind(cmd_buffer);

        // Bind the descriptor sets
        let pipeline_layout = pipeline.layout();
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            0,
            resources.descriptor_set(FixedRenderObjects::CAMERA_DESCRIPTOR_SET),
        );

        // Bind the push constants
        let mesh = match render_data.skybox() {
            &RenderSkybox::Procedural {
                mesh,
                sky_color,
                sun_size,
                sun_direction,
                sun_sharpness,
                sun_color,
                atmosphere_thickness,
                ground_color,
            } => {
                let push_constants = PushConstants {
                    sky_color,
                    sun_size,
                    sun_direction,
                    sun_sharpness,
                    sun_color,
                    atmosphere_thickness,
                    ground_color,
                };
                cmd_buffer.cmd_push_constants(
                    pipeline_layout,
                    VulkanShaderStageFlag::Fragment,
                    0,
                    unsafe { as_bytes(&push_constants) },
                );

                mesh
            }
            _ => unreachable!(),
        };

        // Bind the mesh
        let mesh = render_objects.mesh(mesh);
        mesh.bind(cmd_buffer);

        cmd_buffer.cmd_draw_indexed(mesh.index_count(), 1, 0, 0, 0);
    }
}
