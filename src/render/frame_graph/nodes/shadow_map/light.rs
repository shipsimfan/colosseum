use alexandria::math::Vector2u;

use crate::render::{
    FixedRenderObjects, LightingData, LocalDataBuffer, RenderDirectionalLight, RenderSpotLight,
    frame_graph::FrameGraphResourceId,
};

/// A light that can be used in a shadow map
pub(in crate::render::frame_graph) trait ShadowMapLight:
    Sized
{
    /// The name of this type of light
    const NAME: &str;

    /// Is this light the primary type (i.e. the one in charge of making fixed objects)
    const PRIMARY: bool;

    /// The index of the shadow map buffer associated with this light
    const SHADOW_MAP: usize;

    /// The ID of the shadow map resource associated with this light
    const RESOURCE_ID: FrameGraphResourceId;

    /// The index of the descriptor set associated with this light
    const DESCRIPTOR_SET: usize;

    /// The size of a single shadow map for this light
    const SIZE: Vector2u;

    /// The number of cascades for this light's shadow map
    const CASCADES: usize;

    /// Get the list of lights from the lighting data
    fn get_lights(lighting: &LightingData) -> &LocalDataBuffer<Self>;
}

impl ShadowMapLight for RenderDirectionalLight {
    const NAME: &str = "Directional";
    const PRIMARY: bool = true;
    const SHADOW_MAP: usize = FixedRenderObjects::DIRECTIONAL_LIGHT_SHADOW_MAPS;
    const RESOURCE_ID: FrameGraphResourceId = FrameGraphResourceId::DIRECTIONAL_LIGHT_SHADOW_MAPS;
    const DESCRIPTOR_SET: usize = FixedRenderObjects::DIRECTIONAL_LIGHT_DESCRIPTOR_SET;

    const SIZE: Vector2u = (1024, 1024).into();
    const CASCADES: usize = 4;

    fn get_lights(lighting: &LightingData) -> &LocalDataBuffer<Self> {
        lighting.directional_lights()
    }
}

impl ShadowMapLight for RenderSpotLight {
    const NAME: &str = "Spot";
    const PRIMARY: bool = false;
    const SHADOW_MAP: usize = FixedRenderObjects::SPOT_LIGHT_SHADOW_MAPS;
    const RESOURCE_ID: FrameGraphResourceId = FrameGraphResourceId::SPOT_LIGHT_SHADOW_MAPS;
    const DESCRIPTOR_SET: usize = FixedRenderObjects::SPOT_LIGHT_DESCRIPTOR_SET;

    const SIZE: Vector2u = (1024, 1024).into();
    const CASCADES: usize = 1;

    fn get_lights(lighting: &LightingData) -> &LocalDataBuffer<Self> {
        lighting.spot_lights()
    }
}
