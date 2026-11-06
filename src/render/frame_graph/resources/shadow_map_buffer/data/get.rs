use crate::render::frame_graph::ShadowMapData;

impl ShadowMapData {
    /// Get the number of cascades for the shadow map
    pub fn cascades(&self) -> u32 {
        self.cascades
    }

    /// Is this shadow map a cube map?
    pub fn cube_map(&self) -> bool {
        self.cube_map
    }
}
