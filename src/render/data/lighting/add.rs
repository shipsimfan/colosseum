use crate::render::{LightingData, RenderDirectionalLight, RenderPointLight, RenderSpotLight};
use alexandria::math::Matrix4x4f;

impl LightingData {
    /// Add a new directional light to the data
    pub fn add_directional_light(
        &mut self,
        directional_light: RenderDirectionalLight,
        view_projections: [Matrix4x4f; 4],
    ) {
        self.directional_lights.push(directional_light);
        for view_projection in view_projections {
            self.directional_light_matrices.push(view_projection);
        }
    }

    /// Add a new point light to the data
    pub fn add_point_light(&mut self, point_light: RenderPointLight) {
        self.point_lights.push(point_light);
    }

    /// Add a new point light to the data
    pub fn add_spot_light(&mut self, spot_light: RenderSpotLight, view_projection: Matrix4x4f) {
        self.spot_lights.push(spot_light);
        self.spot_light_matrices.push(view_projection);
    }
}
