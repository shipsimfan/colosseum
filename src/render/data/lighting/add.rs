use crate::render::{LightingData, RenderDirectionalLight, RenderPointLight, RenderSpotLight};

impl LightingData {
    /// Add a new directional light to the data
    pub fn add_directional_light(&mut self, directional_light: RenderDirectionalLight) {
        self.directional_lights.push(directional_light);
    }

    /// Add a new point light to the data
    pub fn add_point_light(&mut self, point_light: RenderPointLight) {
        self.point_lights.push(point_light);
    }

    /// Add a new point light to the data
    pub fn add_spot_light(&mut self, spot_light: RenderSpotLight) {
        self.spot_lights.push(spot_light);
    }
}
