use crate::{
    Error, Result,
    render::{
        AntiAliasingMode, LightingData, LocalDataBuffer, RenderCamera, RenderData, RenderSkybox,
    },
};
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice};
#[cfg(debug_assertions)]
use std::ffi::CString;
use std::sync::Arc;

impl RenderData {
    /// The initial capacity for the renderable data buffer
    pub(in crate::render) const RENDERABLE_BUFFER_INIT_CAPACITY: usize = 256;

    /// Create a new set of [`RenderData`]
    pub(in crate::render) fn new(
        index: usize,
        device: &VulkanDevice,
        memory_properties: &Arc<VulkanAdapterMemoryProperties>,
    ) -> Result<RenderData> {
        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut copy_fence = device.create_fence(0).map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut copy_fence,
                &CString::new(format!("Copy Fence {}", index)).unwrap(),
            )
            .map_err(Error::new_inner)?;

        let mut camera = LocalDataBuffer::new(
            format!("Camera Local Buffer {}", index),
            1,
            device,
            memory_properties,
        )?;
        camera.push(RenderCamera::new());

        let lighting = LightingData::new(index, device, memory_properties)?;
        let renderable_buffer = LocalDataBuffer::new(
            format!("Renderable Local Buffer {}", index),
            RenderData::RENDERABLE_BUFFER_INIT_CAPACITY,
            device,
            memory_properties,
        )?;

        Ok(RenderData {
            render_object_changes: Vec::new(),
            confirmed_removals: Vec::new(),
            copy_fence,
            copy_commands_sent: false,

            render_scale: 1.0,
            gamma: 2.2,
            exposure: 1.0,
            contrast: 1.0,
            saturation: 1.0,
            anti_aliasing: AntiAliasingMode::None,

            skybox: RenderSkybox::default(),
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
