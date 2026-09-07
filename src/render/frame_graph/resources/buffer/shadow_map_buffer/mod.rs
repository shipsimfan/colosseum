use alexandria::{
    gpu::{VulkanDeviceMemory, VulkanFormat, VulkanImage, VulkanImageView},
    math::Vector2u,
};

mod new;

/// A buffer for holding shadow maps
pub struct ShadowMapBuffer {
    /// The GPU image containing the object data
    image: VulkanImage,

    /// The memory the image uses
    #[allow(unused)]
    memory: VulkanDeviceMemory,

    /// The image view covering the entire array
    complete_image_view: VulkanImageView,

    /// The image views covering the individual layers of the array
    layer_image_views: Vec<VulkanImageView>,

    /// The size of the individual shadow maps
    size: Vector2u,
}

impl ShadowMapBuffer {
    pub const FORMAT: VulkanFormat = VulkanFormat::D32SFloat;
}
