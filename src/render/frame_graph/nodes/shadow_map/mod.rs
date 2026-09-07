mod create_fixed_objects;
mod create_per_frame_objects;
mod execute;
mod new;
mod usages;

/// A node that renders shadow maps for lights
#[derive(Debug)]
pub(in crate::render::frame_graph) struct ShadowMapNode {}
