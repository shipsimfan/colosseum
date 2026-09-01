use crate::render::LocalDataBuffer;
use metadata::*;

mod directional;
mod metadata;
mod point;
mod spot;

mod add;
mod new;
mod reserve;
mod reset;
mod set;

pub(crate) use directional::*;
pub(crate) use point::*;
pub(crate) use spot::*;

/// The data about lighting for a given frame
pub(crate) struct LightingData {
    /// The metadata describing the lighting
    metadata: LocalDataBuffer<LightingMetadata>,

    /// The buffer containing the directional light data
    directional_lights: LocalDataBuffer<RenderDirectionalLight>,

    /// The buffer containing the point light data
    point_lights: LocalDataBuffer<RenderPointLight>,

    /// The buffer containing the spot light data
    spot_lights: LocalDataBuffer<RenderSpotLight>,
}
