use crate::render::frame_graph::ShadowMapData;

impl ShadowMapData {
    /// Create a new set of [`ShadowMapData`]
    pub fn new(cascades: u32, cube_map: bool) -> ShadowMapData {
        ShadowMapData { cascades, cube_map }
    }
}
