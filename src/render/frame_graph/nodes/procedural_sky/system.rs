use crate::render::{
    Assets, FrameIndex, Pipeline, Primitives, SecondaryCommandBuffer, Skybox,
    frame_graph::{FrameGraphResources, ProceduralSkyNode, nodes::procedural_sky::PushConstants},
};
use alexandria::gpu::{VulkanPipelineBindPoint, VulkanShaderStageFlag};
use colosseum_core::as_bytes;
use colosseum_ecs::{Res, ResMut};

impl ProceduralSkyNode {
    /// Clear the sky with a solid color
    pub(in crate::render::frame_graph::nodes::procedural_sky) fn system(
        self: Res<Self>,
        skybox: Res<Skybox>,
        pipeline: Res<Pipeline<ProceduralSkyNode>>,
        frame_index: Res<FrameIndex>,
        resources: Res<FrameGraphResources>,
        assets: Res<Assets>,
        primitives: Res<Primitives>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        let cmd_buffer = &mut command_buffer[&frame_index];

        // Bind the pipeline
        pipeline.bind(cmd_buffer);

        // Bind the descriptor sets
        let pipeline_layout = pipeline.layout();
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            0,
            resources.descriptor_set(self.descriptor_set, &frame_index),
        );

        // Bind the push constants
        match skybox.as_ref() {
            Skybox::Procedural(procedural) => {
                let push_constants = PushConstants {
                    sky_color: procedural.sky_color(),
                    sun_size: procedural.sun_size(),
                    sun_direction: procedural.sun_direction(),
                    sun_sharpness: procedural.sun_sharpness(),
                    sun_color: procedural.sun_color(),
                    atmosphere_thickness: procedural.atmosphere_thickness(),
                    ground_color: procedural.ground_color(),
                };
                cmd_buffer.cmd_push_constants(
                    pipeline_layout,
                    VulkanShaderStageFlag::Fragment,
                    0,
                    as_bytes(&push_constants),
                );
            }
            _ => unreachable!(),
        };

        // Bind the mesh
        let mesh = assets.get_mesh(primitives.cube());
        mesh.bind(cmd_buffer);

        cmd_buffer.cmd_draw_indexed(mesh.indices().len() as _, 1, 0, 0, 0);
    }
}
