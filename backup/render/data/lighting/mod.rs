use crate::render::LocalDataBuffer;
use alexandria::math::Matrix4x4f;

mod directional;
mod metadata;
mod point;
mod shadow_quality;
mod spot;

mod add;
mod get;
mod new;
mod reserve;
mod reset;
mod set;

pub use shadow_quality::*;

pub(crate) use directional::*;
pub(crate) use point::*;
pub(crate) use spot::*;

pub(in crate::render) use metadata::*;

/// The data about lighting for a given frame
pub(crate) struct LightingData {
    /// The metadata describing the lighting
    metadata: LocalDataBuffer<LightingMetadata>,

    /// The buffer containing the directional light data
    directional_lights: LocalDataBuffer<RenderDirectionalLight>,

    /// The buffer containing the view-projection matrices of the directional lights
    directional_light_matrices: LocalDataBuffer<Matrix4x4f>,

    /// The buffer containing the point light data
    point_lights: LocalDataBuffer<RenderPointLight>,

    /// The buffer containing the view-projection matrices of the point lights
    point_light_matrices: LocalDataBuffer<Matrix4x4f>,

    /// The buffer containing the spot light data
    spot_lights: LocalDataBuffer<RenderSpotLight>,

    /// The buffer containing the view-projection matrices of the spot lights
    spot_light_matrices: LocalDataBuffer<Matrix4x4f>,

    /// The quality to use for shadow maps
    shadow_quality: ShadowQuality,
}
