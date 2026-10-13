#![allow(private_interfaces)]
use crate::render::{
    FixedRenderObjects, LightingData, LocalDataBuffer, RenderDirectionalLight, RenderPointLight,
    RenderSpotLight, frame_graph::FrameGraphResourceId,
};
use alexandria::math::Vector2u;

/// A light that can be used in a shadow map
pub(crate) trait ShadowMapLight: Sized {
    /// The name of this type of light
    const NAME: &str;

    /// Is this light the primary type (i.e. the one in charge of making fixed objects)
    const PRIMARY: bool;

    /// The index of the shadow map buffer associated with this light
    const SHADOW_MAP: usize;

    /// The ID of the shadow map resource associated with this light
    const RESOURCE_ID: FrameGraphResourceId;

    /// The index of the descriptor set layout associated with this light
    const DESCRIPTOR_SET_LAYOUT: usize;

    /// The index of the descriptor set associated with this light
    const DESCRIPTOR_SET: usize;

    /// The index of the pipeline associated with this light
    const PIPELINE: usize;

    /// The size of a single shadow map for this light, one for each quality level
    const SIZE: [Vector2u; 3];

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
    const DESCRIPTOR_SET_LAYOUT: usize = FixedRenderObjects::SHADOW_MAP_DESCRIPTOR_SET_LAYOUT;
    const DESCRIPTOR_SET: usize = FixedRenderObjects::DIRECTIONAL_LIGHT_DESCRIPTOR_SET;
    const PIPELINE: usize = FixedRenderObjects::SHADOW_MAP_PIPELINE;

    const SIZE: [Vector2u; 3] = [(512, 512).into(), (1024, 1024).into(), (2048, 2048).into()];
    const CASCADES: usize = 4;

    fn get_lights(lighting: &LightingData) -> &LocalDataBuffer<Self> {
        lighting.directional_lights()
    }
}

impl ShadowMapLight for RenderPointLight {
    const NAME: &str = "Point";
    const PRIMARY: bool = false;
    const SHADOW_MAP: usize = FixedRenderObjects::POINT_LIGHT_SHADOW_MAPS;
    const RESOURCE_ID: FrameGraphResourceId = FrameGraphResourceId::POINT_LIGHT_SHADOW_MAPS;
    const DESCRIPTOR_SET_LAYOUT: usize =
        FixedRenderObjects::POINT_LIGHT_SHADOW_MAP_DESCRIPTOR_SET_LAYOUT;
    const DESCRIPTOR_SET: usize = FixedRenderObjects::POINT_LIGHT_DESCRIPTOR_SET;
    const PIPELINE: usize = FixedRenderObjects::POINT_LIGHT_SHADOW_MAP_PIPELINE;

    const SIZE: [Vector2u; 3] = [(256, 256).into(), (512, 512).into(), (1024, 1024).into()];
    const CASCADES: usize = 6;

    fn get_lights(lighting: &LightingData) -> &LocalDataBuffer<Self> {
        lighting.point_lights()
    }
}

impl ShadowMapLight for RenderSpotLight {
    const NAME: &str = "Spot";
    const PRIMARY: bool = false;
    const SHADOW_MAP: usize = FixedRenderObjects::SPOT_LIGHT_SHADOW_MAPS;
    const RESOURCE_ID: FrameGraphResourceId = FrameGraphResourceId::SPOT_LIGHT_SHADOW_MAPS;
    const DESCRIPTOR_SET_LAYOUT: usize = FixedRenderObjects::SHADOW_MAP_DESCRIPTOR_SET_LAYOUT;
    const DESCRIPTOR_SET: usize = FixedRenderObjects::SPOT_LIGHT_DESCRIPTOR_SET;
    const PIPELINE: usize = FixedRenderObjects::SHADOW_MAP_PIPELINE;

    const SIZE: [Vector2u; 3] = [(512, 512).into(), (1024, 1024).into(), (2048, 2048).into()];
    const CASCADES: usize = 1;

    fn get_lights(lighting: &LightingData) -> &LocalDataBuffer<Self> {
        lighting.spot_lights()
    }
}
