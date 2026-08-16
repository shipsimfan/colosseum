use std::ptr::addr_of;

use crate::render::{MaterialKind, RenderData, RenderObjects, frame_graph::UnlitForwardRenderNode};
use alexandria::{
    gpu::{
        GpuAddress, VulkanCommandBuffer, VulkanPipelineBindPoint, VulkanShaderStageFlag,
        VulkanViewport,
    },
    math::{Matrix4x4f, Recti, Vector2, Vector2u},
};

impl UnlitForwardRenderNode {
    /// Execute the unlit forward pass, rendering all unlit objects in the scene
    pub(in crate::render::frame_graph) fn execute(
        &self,
        render_data: &RenderData,
        swapchain_size: Vector2u,
        cmd_buffer: &mut VulkanCommandBuffer,
        render_objects: &RenderObjects,
    ) {
        // Bind the viewport and scissor for the render pass
        let viewport = VulkanViewport::new(
            0.0,
            0.0,
            swapchain_size.x as _,
            swapchain_size.y as _,
            0.0,
            1.0,
        );
        let scissor = Recti::new(Vector2::ZERO, swapchain_size);
        cmd_buffer.cmd_set_viewport(0, &[viewport]);
        cmd_buffer.cmd_set_scissor(0, &[scissor]);

        // Bind the camera descriptor set
        let pipeline_layout = render_objects.material_pipeline_layout(MaterialKind::UnlitOpaque);
        cmd_buffer.cmd_bind_descriptor_set(
            VulkanPipelineBindPoint::Graphics,
            pipeline_layout,
            0,
            render_data.camera().descriptor_set(),
        );

        for (material, mesh, object_data) in render_data.unlit_opaque_renderables() {
            let material = render_objects.unlit_opaque_material(material);
            let mesh = render_objects.mesh(mesh);

            material.bind(cmd_buffer);
            mesh.bind(cmd_buffer);

            cmd_buffer.cmd_push_constants(
                pipeline_layout,
                VulkanShaderStageFlag::Vertex,
                0,
                unsafe {
                    std::slice::from_raw_parts(
                        addr_of!(object_data).cast(),
                        std::mem::size_of::<GpuAddress<Matrix4x4f>>(),
                    )
                },
            );

            cmd_buffer.cmd_draw(mesh.index_count(), 1, 0, 0);
        }
    }
}
