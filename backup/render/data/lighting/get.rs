use crate::render::{
    LightingData, LightingMetadata, LocalDataBuffer, RenderDirectionalLight, RenderPointLight,
    RenderSpotLight, ShadowQuality,
};
use alexandria::math::Matrix4x4f;

impl LightingData {
    /// Get a reference to the metadata buffer
    pub(in crate::render) fn metadata(&self) -> &LocalDataBuffer<LightingMetadata> {
        &self.metadata
    }

    /// Get a reference to the directional lights buffer
    pub(in crate::render) fn directional_lights(&self) -> &LocalDataBuffer<RenderDirectionalLight> {
        &self.directional_lights
    }

    /// Get a reference to the directional light matrix buffer
    pub(in crate::render) fn directional_light_matrices(&self) -> &LocalDataBuffer<Matrix4x4f> {
        &self.directional_light_matrices
    }

    /// Get a reference to the point lights buffer
    pub(in crate::render) fn point_lights(&self) -> &LocalDataBuffer<RenderPointLight> {
        &self.point_lights
    }

    /// Get a reference to the point light matrix buffer
    pub(in crate::render) fn point_light_matrices(&self) -> &LocalDataBuffer<Matrix4x4f> {
        &self.point_light_matrices
    }

    /// Get a reference to the spot lights buffer
    pub(in crate::render) fn spot_lights(&self) -> &LocalDataBuffer<RenderSpotLight> {
        &self.spot_lights
    }

    /// Get a reference to the spot light matrix buffer
    pub(in crate::render) fn spot_light_matrices(&self) -> &LocalDataBuffer<Matrix4x4f> {
        &self.spot_light_matrices
    }

    /// Get the shadow quality
    pub fn shadow_quality(&self) -> ShadowQuality {
        self.shadow_quality
    }
}
