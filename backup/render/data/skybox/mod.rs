use crate::render::Mesh;
use alexandria::{
    Id,
    math::{Color3f, Linear, Vector3f},
};

mod default;

/// A skybox that can be rendered in the scene
#[derive(Clone)]
pub enum RenderSkybox {
    /// Clear the screen to a solid color before rendering the scene
    SolidColor(Color3f<Linear>),

    /// A procedurally generated skybox
    Procedural {
        /// The ID of the cube mesh used for rendering
        mesh: Id<Mesh>,

        /// The color of the sky
        sky_color: Color3f<Linear>,

        /// The size of the sun
        sun_size: f32,

        /// The direction of the sun
        sun_direction: Vector3f,

        /// The sharpness of the sun's appearance
        sun_sharpness: f32,

        /// The color of the sun
        sun_color: Color3f<Linear>,

        /// The thickness of the atmosphere
        atmosphere_thickness: f32,

        /// The color of the ground
        ground_color: Color3f<Linear>,
    },
}
