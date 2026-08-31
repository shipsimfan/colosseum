use crate::{
    Result,
    render::{
        LightingData, RenderDirectionalLight, RenderPointLight, RenderSpotLight,
        data::lighting::LightingDataBuffer,
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanDescriptorBufferInfo, VulkanDescriptorType, VulkanDevice,
    VulkanWriteDescriptorSet,
};

impl LightingData {
    /// Reserve enough space for the specified number of directional lights
    pub fn reserve_directional_lights(
        &mut self,
        directional_lights: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        self.metadata[0].num_directional_lights = directional_lights as _;

        if self.directional_lights.capacity() > directional_lights {
            return Ok(());
        }

        let mut new_capacity = self.directional_lights.capacity();
        while new_capacity < directional_lights {
            new_capacity *= 2;
        }

        self.directional_lights = LightingDataBuffer::new(new_capacity, device, memory_properties)?;

        // Update the descriptor set with the buffers
        device.update_descriptor_sets(
            &[VulkanWriteDescriptorSet::new(
                &self.descriptor_set,
                1,
                0,
                VulkanDescriptorType::StorageBuffer,
                &[],
                &[VulkanDescriptorBufferInfo::new(
                    &self.directional_lights.buffer(),
                    0,
                    (std::mem::size_of::<RenderDirectionalLight>()
                        * self.directional_lights.capacity()) as _,
                )],
            )],
            &[],
        );

        Ok(())
    }

    /// Reserve enough space for the specified number of point lights
    pub fn reserve_point_lights(
        &mut self,
        point_lights: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        self.metadata[0].num_point_lights = point_lights as _;

        if self.point_lights.capacity() > point_lights {
            return Ok(());
        }

        let mut new_capacity = self.point_lights.capacity();
        while new_capacity < point_lights {
            new_capacity *= 2;
        }

        self.point_lights = LightingDataBuffer::new(new_capacity, device, memory_properties)?;

        // Update the descriptor set with the buffers
        device.update_descriptor_sets(
            &[VulkanWriteDescriptorSet::new(
                &self.descriptor_set,
                2,
                0,
                VulkanDescriptorType::StorageBuffer,
                &[],
                &[VulkanDescriptorBufferInfo::new(
                    &self.point_lights.buffer(),
                    0,
                    (std::mem::size_of::<RenderPointLight>() * self.point_lights.capacity()) as _,
                )],
            )],
            &[],
        );

        Ok(())
    }

    /// Reserve enough space for the specified number of spot lights
    pub fn reserve_spot_lights(
        &mut self,
        spot_lights: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        self.metadata[0].num_spot_lights = spot_lights as _;

        if self.spot_lights.capacity() > spot_lights {
            return Ok(());
        }

        let mut new_capacity = self.spot_lights.capacity();
        while new_capacity < spot_lights {
            new_capacity *= 2;
        }

        self.spot_lights = LightingDataBuffer::new(new_capacity, device, memory_properties)?;

        // Update the descriptor set with the buffers
        device.update_descriptor_sets(
            &[VulkanWriteDescriptorSet::new(
                &self.descriptor_set,
                3,
                0,
                VulkanDescriptorType::StorageBuffer,
                &[],
                &[VulkanDescriptorBufferInfo::new(
                    &self.spot_lights.buffer(),
                    0,
                    (std::mem::size_of::<RenderSpotLight>() * self.spot_lights.capacity()) as _,
                )],
            )],
            &[],
        );

        Ok(())
    }
}
