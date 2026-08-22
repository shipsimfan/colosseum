use crate::{
    Result,
    render::{FixedRenderObjects, GpuTransferQueue, Shader, ShaderId, ShaderKind},
    update::{UpdateRenderObjects, render_objects::GpuAllocator},
};
use alexandria::{
    SlotMap,
    gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanFormat, compile_shader},
};
use std::sync::Arc;

compile_shader! {
    /// The vertex shader code for unlit objects
    const UNLIT_SHADER = "unlit-opaque.slang",
    vert_main,
    frag_main
}

impl UpdateRenderObjects {
    /// Create a new set of [`UpdateRenderObjects`]
    pub(in crate::update) fn new(
        device: VulkanDevice,
        swapchain_format: VulkanFormat,
        transfer_queue: GpuTransferQueue,
        memory_properties: Arc<VulkanAdapterMemoryProperties>,
        fixed_render_objects: Arc<FixedRenderObjects>,
    ) -> Result<UpdateRenderObjects> {
        let mesh_allocator = GpuAllocator::new(
            16 * 1024 * 1024, // 16 MB
            256,              // 256 B
            4 * 1024 * 1024,  // 4 MB
            memory_properties,
            device.clone(),
        );

        let mut unlit_shaders = SlotMap::new();
        let default_unlit_shader = ShaderId::new(
            ShaderKind::Unlit,
            unlit_shaders.insert(Shader::new(&UNLIT_SHADER, &device)?),
        );

        Ok(UpdateRenderObjects {
            device,
            fixed_render_objects,
            swapchain_format,
            mesh_allocator,
            meshes: SlotMap::new(),
            unlit_shaders,
            default_unlit_shader,
            unlit_opaque_materials: SlotMap::new(),
            transfer_queue,
        })
    }
}
