mod get;
mod new;

/// Extra data associated with a shadow map resource
pub(in crate::render::frame_graph) struct ShadowMapData {
    /// The number of cascades for the shadow map
    cascades: u32,

    /// Is this shadow map a cube map?
    cube_map: bool,
}
