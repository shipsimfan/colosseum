use crate::graphics::{
    context::Lights,
    lights::{DirectionalLightHandle, PointLightHandle, SpotLightHandle},
};

impl Lights {
    /// Remove `directional_light` from the set of lights
    pub fn remove_directional_light(&mut self, directional_light: DirectionalLightHandle) {
        self.directional_lights.remove(directional_light);
    }

    /// Remove `point_light` from the set of lights
    pub fn remove_point_light(&mut self, point_light: PointLightHandle) {
        self.point_lights.remove(point_light);
    }

    /// Remove `spot_light` from the set of lights
    pub fn remove_spot_light(&mut self, spot_light: SpotLightHandle) {
        self.spot_lights.remove(spot_light);
    }
}
