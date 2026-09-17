use crate::{
    Result,
    render::{
        LightingData,
        data::{LocalDataBuffer, lighting::LightingMetadata},
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

impl LightingData {
    pub const INITIAL_DIRECTIONAL_LIGHT_CAPACITY: usize = 1;
    pub const INITIAL_POINT_LIGHT_CAPACITY: usize = 32;
    pub const INITIAL_SPOT_LIGHT_CAPACITY: usize = 8;

    /// Create a new set of [`LightingData`]
    pub(in crate::render::data) fn new(
        index: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<LightingData> {
        // Allocate data buffers
        let mut metadata = LocalDataBuffer::new(
            format!("Lighting Metadata Local Buffer {}", index),
            1,
            device,
            memory_properties,
        )?;
        metadata.push(LightingMetadata::default());

        let directional_lights = LocalDataBuffer::new(
            format!("Directional Light Local Buffer {}", index),
            LightingData::INITIAL_DIRECTIONAL_LIGHT_CAPACITY,
            device,
            memory_properties,
        )?;
        let directional_light_matrices = LocalDataBuffer::new(
            format!("Directional Light Matrix Local Buffer {}", index),
            LightingData::INITIAL_DIRECTIONAL_LIGHT_CAPACITY * 4,
            device,
            memory_properties,
        )?;
        let point_lights = LocalDataBuffer::new(
            format!("Point Light Local Buffer {}", index),
            LightingData::INITIAL_POINT_LIGHT_CAPACITY,
            device,
            memory_properties,
        )?;
        let point_light_matrices = LocalDataBuffer::new(
            format!("Point Light Matrix Local Buffer {}", index),
            LightingData::INITIAL_POINT_LIGHT_CAPACITY * 6,
            device,
            memory_properties,
        )?;
        let spot_lights = LocalDataBuffer::new(
            format!("Spot Light Local Buffer {}", index),
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            device,
            memory_properties,
        )?;
        let spot_light_matrices = LocalDataBuffer::new(
            format!("Spot Light Matrix Local Buffer {}", index),
            LightingData::INITIAL_SPOT_LIGHT_CAPACITY,
            device,
            memory_properties,
        )?;

        Ok(LightingData {
            metadata,
            directional_lights,
            directional_light_matrices,
            point_lights,
            point_light_matrices,
            spot_lights,
            spot_light_matrices,
        })
    }
}
