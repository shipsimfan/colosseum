#[cfg(debug_assertions)]
use std::ffi::CString;

use crate::{
    Error, Result,
    render::{
        DeviceBufferDescriptorSet, DeviceDataBuffer, FixedRenderObjects, PerFrameObjectBuilder,
        ShadowMapBuffer,
    },
};
use alexandria::{
    gpu::{VulkanBufferUsageFlags, VulkanDescriptorType},
    math::Vector2u,
};

impl<'a> PerFrameObjectBuilder<'a> {
    /// Add a new per-frame descriptor set
    pub fn add_descriptor_set(
        &mut self,
        #[cfg_attr(not(debug_assertions), allow(unused_variables))] name: String,
        descriptor_set_layout: usize,
        index: usize,
    ) -> Result<()> {
        assert_eq!(index, self.descriptor_sets.len());

        let descriptor_set_layout = self
            .fixed_render_objects
            .descriptor_set_layout(descriptor_set_layout);

        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut descriptor_set = self
            .descriptor_pool
            .allocate_descriptor_set(descriptor_set_layout)
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        self.device
            .set_object_name(&mut descriptor_set, &CString::new(name).unwrap())
            .map_err(Error::new_inner)?;

        self.descriptor_sets.push(descriptor_set);
        Ok(())
    }

    /// Add a new per-frame device data buffer
    pub fn add_device_data_buffer<T, U: Into<VulkanBufferUsageFlags>>(
        &mut self,
        name: String,
        initial_capacity: usize,
        usage: U,
        descriptor_type: VulkanDescriptorType,
        descriptor_sets: Vec<DeviceBufferDescriptorSet>,

        index: usize,
    ) -> Result<()> {
        assert_eq!(index, self.device_buffers.len());

        let device_buffer = DeviceDataBuffer::new::<T>(
            name,
            initial_capacity,
            usage.into(),
            descriptor_type,
            descriptor_sets,
            self.descriptor_sets,
            self.device,
            self.memory_properties,
        )?;
        self.device_buffers.push(device_buffer);

        Ok(())
    }

    /// Add a new [`ShadowMapBuffer`]
    pub fn add_shadow_map_buffer<V: Into<Vector2u>>(
        &mut self,
        name: String,
        size: V,
        count: usize,
        cascades: usize,
        cube: bool,
        descriptor_set: usize,
        binding: u32,
        index: usize,
    ) -> Result<()> {
        assert_eq!(index, self.shadow_map_buffers.len());

        let shadow_map_buffer = ShadowMapBuffer::new(
            name,
            size.into(),
            count,
            cascades,
            cube,
            descriptor_set,
            binding,
            self.descriptor_sets,
            self.fixed_render_objects
                .sampler(FixedRenderObjects::LINEAR_CLAMP_SAMPLER),
            self.device,
            self.memory_properties,
        )?;
        self.shadow_map_buffers.push(shadow_map_buffer);

        Ok(())
    }
}
