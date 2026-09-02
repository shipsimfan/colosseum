use crate::render::{
    LightingData, LightingMetadata, LocalDataBuffer, RenderDirectionalLight, RenderPointLight,
    RenderSpotLight,
};

impl LightingData {
    /// Get a reference to the metadata buffer
    pub(in crate::render) fn metadata(&self) -> &LocalDataBuffer<LightingMetadata> {
        &self.metadata
    }

    /// Get a reference to the directional lights buffer
    pub(in crate::render) fn directional_lights(&self) -> &LocalDataBuffer<RenderDirectionalLight> {
        &self.directional_lights
    }

    /// Get a reference to the point lights buffer
    pub(in crate::render) fn point_lights(&self) -> &LocalDataBuffer<RenderPointLight> {
        &self.point_lights
    }

    /// Get a reference to the spot lights buffer
    pub(in crate::render) fn spot_lights(&self) -> &LocalDataBuffer<RenderSpotLight> {
        &self.spot_lights
    }
}
