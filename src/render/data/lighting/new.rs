use crate::{
    Result,
    render::{
        LightingData,
        data::{LocalDataBuffer, lighting::LightingMetadata},
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};

const INITIAL_DIRECTIONAL_LIGHT_CAPACITY: usize = 1;
const INITIAL_POINT_LIGHT_CAPACITY: usize = 32;
const INITIAL_SPOT_LIGHT_CAPACITY: usize = 8;

impl LightingData {
    /// Create a new set of [`LightingData`]
    pub(in crate::render::data) fn new(
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<LightingData> {
        // Allocate data buffers
        let mut metadata = LocalDataBuffer::new(1, device, memory_properties)?;
        metadata.push(LightingMetadata::default());

        let directional_lights = LocalDataBuffer::new(
            INITIAL_DIRECTIONAL_LIGHT_CAPACITY,
            device,
            memory_properties,
        )?;
        let point_lights =
            LocalDataBuffer::new(INITIAL_POINT_LIGHT_CAPACITY, device, memory_properties)?;
        let spot_lights =
            LocalDataBuffer::new(INITIAL_SPOT_LIGHT_CAPACITY, device, memory_properties)?;

        Ok(LightingData {
            metadata,
            directional_lights,
            point_lights,
            spot_lights,
        })
    }
}
