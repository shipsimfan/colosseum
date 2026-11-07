use alexandria::{
    Id,
    math::{Color3f, Linear, Vector3f},
};
use colosseum_ecs::Entity;

mod get;
mod new;
mod set;
mod system;

/// The parameters controlling a procedural skybox
#[derive(Clone)]
pub struct ProceduralSkybox {
    /// The base color of the sky
    sky_color: Color3f<Linear>,

    /// The direcitonal light to follow
    directional_light: Option<Id<Entity>>,

    /// The direction of the sun
    sun_direction: Vector3f,

    /// The size of the sun
    sun_size: f32,

    /// The sharpness of the sun's appearance
    sun_sharpness: f32,

    /// The color of the sun
    sun_color: Color3f<Linear>,

    /// The thickness of the atmosphere
    atmosphere_thickness: f32,

    /// The color of the ground
    ground_color: Color3f<Linear>,
}
