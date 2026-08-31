use crate::render::LightingData;

impl LightingData {
    /// Reset the lighting data for a new frame
    pub fn reset(&mut self) {
        self.directional_lights.reset();
        self.point_lights.reset();
        self.spot_lights.reset();
        self.metadata[0].num_directional_lights = 0;
        self.metadata[0].num_point_lights = 0;
        self.metadata[0].num_spot_lights = 0;
    }
}
