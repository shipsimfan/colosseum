use crate::graphics::{
    lights::{DirectionalLight, PointLight, SpotLight},
    util::ConstantBuffer,
};
use cb_content::LightCbContent;
use list::LightList;

mod cb_content;
mod list;

mod bind;
mod create;
mod get;
mod new;
mod remove;
mod set;

pub(in crate::graphics) use list::LightType;

/// The lights registered with the engine
pub(in crate::graphics) struct Lights {
    /// The global information about lighting
    constant_buffer: ConstantBuffer<LightCbContent>,

    /// The list of directional lights
    directional_lights: LightList<DirectionalLight>,

    /// The list of point lights
    point_lights: LightList<PointLight>,

    /// The list of spot lights
    spot_lights: LightList<SpotLight>,
}
