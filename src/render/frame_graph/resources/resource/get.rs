use crate::render::frame_graph::FrameGraphResource;
use alexandria::{
    gpu::{VulkanImage, VulkanImageAspectFlag, VulkanImageAspectFlags, VulkanImageView},
    math::Vector2u,
};

impl<'a, 'b> FrameGraphResource<'a, 'b> {
    /// Get the size of the resource
    pub fn size(&self) -> Vector2u {
        match self {
            FrameGraphResource::External(external) => external.size(),
            FrameGraphResource::Transient(transient) => transient.size(),
            FrameGraphResource::ShadowMap(shadow_map) => shadow_map.size(),
        }
    }

    /// Get the image associated with this resource
    pub fn image(&self) -> &'b VulkanImage {
        match self {
            FrameGraphResource::External(external) => external.image(),
            FrameGraphResource::Transient(transient) => transient.image(),
            FrameGraphResource::ShadowMap(shadow_map) => shadow_map.image(),
        }
    }

    /// Get the image view associated with this resource
    pub fn image_view(&self) -> &'b VulkanImageView {
        match self {
            FrameGraphResource::External(external) => external.image_view(),
            FrameGraphResource::Transient(transient) => transient.image_view(),
            FrameGraphResource::ShadowMap(_) => {
                panic!("cannot get the image view handle of a shadow map")
            }
        }
    }

    /// Get the aspect mask associated with this resource
    pub fn aspect_mask(&self) -> VulkanImageAspectFlags {
        match self {
            FrameGraphResource::External(external) => external.aspect_mask(),
            FrameGraphResource::Transient(transient) => transient.aspect_mask(),
            FrameGraphResource::ShadowMap(_) => VulkanImageAspectFlag::Depth.into(),
        }
    }

    pub fn layer_count(&self) -> u32 {
        match self {
            FrameGraphResource::External(external) => external.layer_count(),
            FrameGraphResource::Transient(transient) => transient.layer_count(),
            FrameGraphResource::ShadowMap(shadow_map) => shadow_map.layer_count(),
        }
    }
}
