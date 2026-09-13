use alexandria::{
    gpu::{VulkanDeviceMemory, VulkanFormat, VulkanImage, VulkanImageView},
    math::Vector2u,
};
use std::ffi::CString;

mod get;
mod new;
mod reserve;

/// A buffer for holding shadow maps
pub(in crate::render) struct ShadowMapBuffer {
    /// The name of the shadow map buffer
    name: CString,

    /// The GPU image containing the object data
    image: VulkanImage,

    /// The memory the image uses
    #[allow(unused)]
    memory: VulkanDeviceMemory,

    /// The image view covering the entire array
    #[allow(unused)]
    complete_image_view: VulkanImageView,

    /// The image views covering the individual layers of the array
    layer_image_views: Vec<VulkanImageView>,

    /// The size of the individual shadow maps
    size: Vector2u,

    /// The number of cascades per shadow map layer
    cascades: usize,

    /// The descriptor set the complete shadow map buffer should be bound to
    descriptor_set: usize,

    /// The binding index in the descriptor set to bind this buffer to
    binding: u32,
}

impl ShadowMapBuffer {
    pub const FORMAT: VulkanFormat = VulkanFormat::D32Sfloat;
}
