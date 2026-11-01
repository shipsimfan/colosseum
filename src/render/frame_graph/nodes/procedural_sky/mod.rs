use alexandria::math::{Color3f, Linear, Vector3f};

mod add;
mod create_resources;
mod node;
mod system;

/// Renders a procedural sky to the output
#[derive(Debug)]
pub(in crate::render::frame_graph) struct ProceduralSkyNode {
    /// The index of the descriptor set used for the camera
    descriptor_set: usize,
}

#[repr(C)]
struct PushConstants {
    sky_color: Color3f<Linear>,
    sun_size: f32,
    sun_direction: Vector3f,
    sun_sharpness: f32,
    sun_color: Color3f<Linear>,
    atmosphere_thickness: f32,
    ground_color: Color3f<Linear>,
}
