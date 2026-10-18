use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::System;
use std::borrow::Cow;

/// A node in the frame graph
pub(in crate::render::frame_graph) trait FrameGraphNode:
    'static + Sized
{
    /// Get the name of the node
    fn name(&self) -> impl Into<Cow<'static, str>>;

    /// The color to be used with the node
    fn color(&self) -> impl Into<Color3f<Srgb>>;

    /// Create the system for this node
    fn system(&self) -> impl 'static + System;
}
