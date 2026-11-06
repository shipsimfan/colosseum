use crate::{
    Result,
    render::{
        FrameIndex, GpuDataBuffers, GpuDirectionalLight, GpuPointLight, GpuSpotLight,
        GraphicsDevice, HostDataBuffer, ShadowQuality, frame_graph::FrameGraphResources,
    },
};
use alexandria::math::Vector2u;

impl FrameGraphResources {
    /// Resize the frame graph resources to match the new swapchain size
    pub fn resize(
        &mut self,
        swapchain_size: Vector2u,
        render_scale: f32,

        data_buffers: &GpuDataBuffers,

        shadow_quality: ShadowQuality,
        directional_lights: &HostDataBuffer<GpuDirectionalLight>,
        point_lights: &HostDataBuffer<GpuPointLight>,
        spot_lights: &HostDataBuffer<GpuSpotLight>,

        frame_index: &FrameIndex,
        device: &GraphicsDevice,
    ) -> Result<()> {
        let directional_lights = &directional_lights[frame_index];
        let point_lights = &point_lights[frame_index];
        let spot_lights = &spot_lights[frame_index];

        // Update data buffer bindings
        for binding in &mut self.data_bindings {
            binding.update(data_buffers, &self.descriptor_sets, frame_index, device);
        }

        let frame_index = **frame_index;

        // Resize native scale resources
        self.native_scale
            .resize(swapchain_size, frame_index, device, &self.descriptor_sets)?;

        // Resize render scale resources
        let render_scale_size = (swapchain_size.into_f32() * render_scale).round();
        self.render_scale.resize(
            (render_scale_size.x as u32, render_scale_size.y as u32).into(),
            frame_index,
            device,
            &self.descriptor_sets,
        )?;

        // Resize shadow maps
        self.shadow_maps[0].resize(
            shadow_quality.directional_light_size(),
            directional_lights.capacity() as _,
            frame_index,
            device,
            &self.descriptor_sets,
        )?;
        self.shadow_maps[1].resize(
            shadow_quality.point_light_size(),
            point_lights.capacity() as _,
            frame_index,
            device,
            &self.descriptor_sets,
        )?;
        self.shadow_maps[2].resize(
            shadow_quality.spot_light_size(),
            spot_lights.capacity() as _,
            frame_index,
            device,
            &self.descriptor_sets,
        )?;

        Ok(())
    }
}
