use crate::{
    Error, Result,
    render::{
        AntiAliasingMode, LightingData, RenderData, Skybox,
        data::{CameraRenderData, LocalDataBuffer},
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanFenceCreateFlag};
use std::sync::Arc;

/// The initial capacity for the renderable data buffer
const RENDERABLE_BUFFER_INIT_CAPACITY: usize = 256;

impl RenderData {
    /// Create a new set of [`RenderData`]
    pub(in crate::render) fn new(
        device: &VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<RenderData> {
        let copy_fence = device
            .create_fence(VulkanFenceCreateFlag::Signalled)
            .map_err(Error::new_inner)?;

        let mut camera = LocalDataBuffer::new(1, device, memory_properties)?;
        camera.push(CameraRenderData::new());

        let lighting = LightingData::new(device, memory_properties)?;
        let renderable_buffer =
            LocalDataBuffer::new(RENDERABLE_BUFFER_INIT_CAPACITY, device, memory_properties)?;

        Ok(RenderData {
            render_object_changes: Vec::new(),
            confirmed_removals: Vec::new(),
            copy_fence,

            render_scale: 1.0,
            gamma: 2.2,
            exposure: 1.0,
            contrast: 1.0,
            saturation: 1.0,
            anti_aliasing: AntiAliasingMode::None,

            skybox: Skybox::default(),
            camera,
            lighting,

            unlit_opaque_renderables: Vec::new(),
            lit_opaque_renderables: Vec::new(),
            renderable_buffer,

            device: device.clone(),
            memory_properties: memory_properties.clone(),
        })
    }
}
