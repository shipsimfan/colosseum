use crate::render::Vertex;
use alexandria::gpu::{
    VulkanFormat, VulkanVertexInputAttributeDescription, VulkanVertexInputBindingDescription,
    VulkanVertexInputRate,
};

impl Vertex {
    /// The binding descriptors for this vertex
    pub(in crate::render) const BINDING_DESCRIPTORS: [VulkanVertexInputBindingDescription; 1] =
        [VulkanVertexInputBindingDescription::new(
            0,
            std::mem::size_of::<Vertex>() as u32,
            VulkanVertexInputRate::Vertex,
        )];

    /// The attribute descriptors for this vertex
    pub(in crate::render) const ATTRIBUTE_DESCRIPTORS: [VulkanVertexInputAttributeDescription; 2] = [
        VulkanVertexInputAttributeDescription::new(
            0,
            0,
            VulkanFormat::R32G32B32SFloat,
            std::mem::offset_of!(Vertex, position) as u32,
        ),
        VulkanVertexInputAttributeDescription::new(
            1,
            0,
            VulkanFormat::R32G32B32SFloat,
            std::mem::offset_of!(Vertex, color) as u32,
        ),
    ];
}
