use crate::render::frame_graph::{FrameGraphNode, QuantizationNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl FrameGraphNode for QuantizationNode {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "Quantization"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (0.95, 0.35, 0.65)
    }

    fn system(&self) -> impl 'static + System {
        QuantizationNode::system.into_system()
    }
}
