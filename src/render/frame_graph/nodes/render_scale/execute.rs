use crate::render::{
    RenderData, RenderObjects,
    frame_graph::{FrameGraphResources, RenderScaleNode},
};
use alexandria::{
    gpu::{
        VulkanCommandBuffer, VulkanFilter, VulkanImageAspectFlag, VulkanImageBlit,
        VulkanImageLayout,
    },
    math::Vector3i,
};

impl RenderScaleNode {
    /// Execute the solid color sky pass, rendering a full-screen quad with the specified clear color
    pub(in crate::render::frame_graph) fn execute(
        &self,
        _: &RenderData,
        _: &RenderObjects,
        resources: &FrameGraphResources,
        cmd_buffer: &mut VulkanCommandBuffer,
    ) {
        let src = resources.get(self.input);
        let dst = resources.get(self.output);

        cmd_buffer.cmd_blit_image(
            src.image(),
            VulkanImageLayout::TransferSrcOptimal,
            dst.image(),
            VulkanImageLayout::TransferDstOptimal,
            &[VulkanImageBlit::new(
                VulkanImageAspectFlag::Color,
                0,
                0,
                1,
                [
                    Vector3i::ZERO,
                    Vector3i::new(src.size().x as _, src.size().y as _, 1),
                ],
                VulkanImageAspectFlag::Color,
                0,
                0,
                1,
                [
                    Vector3i::ZERO,
                    Vector3i::new(dst.size().x as _, dst.size().y as _, 1),
                ],
            )],
            VulkanFilter::Linear,
        );
    }
}
