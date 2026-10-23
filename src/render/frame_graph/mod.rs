use crate::render::GraphicsDevice;
use builder::*;
use nodes::*;
use resources::*;
use std::{marker::PhantomData, sync::Arc};
use structure::*;
use submission::*;

mod builder;
mod nodes;
mod resources;
mod structure;
mod submission;

mod compile;
mod new;
mod prepare;

pub(in crate::render) use nodes::create_frame_graph_resources;

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph<Game: crate::Game> {
    /// The current structure of the frame graph
    structure: Option<FrameGraphStructure>,

    /// The graphics device used for rendering
    device: Arc<GraphicsDevice>,

    /// A marker for the game type
    game: PhantomData<Game>,
}
