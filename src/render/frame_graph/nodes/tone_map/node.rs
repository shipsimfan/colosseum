use crate::render::frame_graph::{FrameGraphNode, ToneMapNode};
use alexandria::math::{Color3f, Srgb};
use colosseum_ecs::{IntoSystem, System};
use std::borrow::Cow;

impl<Game: crate::Game> FrameGraphNode for ToneMapNode<Game> {
    fn name(&self) -> impl Into<Cow<'static, str>> {
        "Tone Map"
    }

    fn color(&self) -> impl Into<Color3f<Srgb>> {
        (1.0, 0.45, 0.30)
    }

    fn system(&self) -> impl 'static + System {
        ToneMapNode::<Game>::system.into_system()
    }
}
