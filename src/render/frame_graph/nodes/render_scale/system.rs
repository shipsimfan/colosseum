use crate::render::{
    FrameIndex, SecondaryCommandBuffer,
    frame_graph::{FrameGraphResources, RenderScaleNode},
};
use alexandria::{
    gpu::{VulkanFilter, VulkanImageAspectFlag, VulkanImageBlit, VulkanImageLayout},
    math::Vector3i,
};
use colosseum_ecs::{Res, ResMut};

impl RenderScaleNode {
    /// Performs tone mapping on the input image
    pub(in crate::render::frame_graph::nodes::render_scale) fn system(
        self: Res<Self>,
        frame_index: Res<FrameIndex>,
        resources: Res<FrameGraphResources>,
        mut command_buffer: ResMut<SecondaryCommandBuffer<Self>>,
    ) {
        let cmd_buffer = &mut command_buffer[&frame_index];

        let input = resources.get(self.input, &frame_index);
        let output = resources.get(self.output, &frame_index);

        cmd_buffer.cmd_blit_image(
            input.image(),
            VulkanImageLayout::TransferSrcOptimal,
            output.image(),
            VulkanImageLayout::TransferDstOptimal,
            &[VulkanImageBlit::new(
                VulkanImageAspectFlag::Color,
                0,
                0,
                1,
                [
                    Vector3i::ZERO,
                    Vector3i::new(input.size().x as _, input.size().y as _, 1),
                ],
                VulkanImageAspectFlag::Color,
                0,
                0,
                1,
                [
                    Vector3i::ZERO,
                    Vector3i::new(output.size().x as _, output.size().y as _, 1),
                ],
            )],
            VulkanFilter::Linear,
        );
    }
}
