use alexandria::{gpu::VulkanImageView, math::Vector2u};

mod external;
mod kind;
mod transient;

mod from;
mod get;

pub(in crate::render::frame_graph) use external::*;
pub(in crate::render::frame_graph) use kind::*;
pub(in crate::render::frame_graph) use transient::*;

/// A single resource within the frame graph
pub(in crate::render::frame_graph) enum FrameGraphResource<'a> {
    /// An external resource
    External(&'a FrameGraphExternalResource),

    /// A transient resource
    Transient {
        info: &'a FrameGraphTransientResourceInfo,
        image_view: &'a VulkanImageView,
        size: Vector2u,
    },
}
