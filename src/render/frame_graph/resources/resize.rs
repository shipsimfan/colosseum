use crate::{
    Result,
    render::frame_graph::{FrameGraphDynamicTransientResourceInfo, FrameGraphResources},
};
use alexandria::{
    gpu::{VulkanAdapterMemoryProperties, VulkanDevice},
    math::Vector2u,
};

impl<'a> FrameGraphResources<'a> {
    /// Resize the resources to a new swapchain size
    pub fn resize(
        &mut self,
        transient_render_scale_info: &[FrameGraphDynamicTransientResourceInfo],
        transient_native_scale_info: &[FrameGraphDynamicTransientResourceInfo],
        swapchain_size: Vector2u,
        render_scale: f32,
        memory_properties: &VulkanAdapterMemoryProperties,
        device: &VulkanDevice,
        new_epoch: u64,
    ) -> Result<()> {
        let render_scale_size: Vector2u = (swapchain_size.into_f32() * render_scale).from_f32();
        self.transient.render_scale.resize(
            transient_render_scale_info,
            render_scale_size,
            device,
            memory_properties,
        )?;
        self.transient.native_scale.resize(
            transient_native_scale_info,
            swapchain_size,
            device,
            memory_properties,
        )?;

        self.transient.epoch = new_epoch;
        Ok(())
    }
}
