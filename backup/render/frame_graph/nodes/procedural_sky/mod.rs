use crate::render::frame_graph::FrameGraphResourceId;
use alexandria::math::{Color3f, Linear, Vector3f};

mod create_fixed_objects;
mod create_per_frame_objects;
mod execute;
mod new;
mod usages;

/// Renders a procedural sky to the output
#[derive(Debug)]
pub(in crate::render::frame_graph) struct ProceduralSkyNode {
    /// The ID of the output color attachment
    output: FrameGraphResourceId,
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
