use crate::graphics::lights::{DirectionalLightInner, PointLightInner, SpotLightInner};
use constant_buffer::LightConstantBuffer;
use list::LightList;

mod constant_buffer;
mod list;

mod bind;
mod get;
mod new;
mod push;
mod set;

pub(in crate::graphics) use list::LightType;

/// The lights registered with the engine
pub(in crate::graphics) struct Lights {
    /// The global information about lighting
    constant_buffer: LightConstantBuffer,

    /// The list of directional lights
    directional_lights: LightList<DirectionalLightInner>,

    /// The list of point lights
    point_lights: LightList<PointLightInner>,

    /// The list of spot lights
    spot_lights: LightList<SpotLightInner>,
}
