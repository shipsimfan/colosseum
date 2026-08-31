use crate::{Result, render::LightingData};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl LightingData {
    /// Reserve enough space for the specified number of directional lights
    pub fn reserve_directional_lights(
        &mut self,
        directional_lights: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        self.metadata[0].num_directional_lights = directional_lights as _;
        self.directional_lights
            .reserve(directional_lights, device, memory_properties)
    }

    /// Reserve enough space for the specified number of point lights
    pub fn reserve_point_lights(
        &mut self,
        point_lights: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        self.metadata[0].num_point_lights = point_lights as _;
        self.point_lights
            .reserve(point_lights, device, memory_properties)
    }

    /// Reserve enough space for the specified number of spot lights
    pub fn reserve_spot_lights(
        &mut self,
        spot_lights: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<()> {
        self.metadata[0].num_spot_lights = spot_lights as _;
        self.spot_lights
            .reserve(spot_lights, device, memory_properties)
    }
}
