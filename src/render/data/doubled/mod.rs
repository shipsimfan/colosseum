use crate::render::{CameraRenderData, Material, Mesh, ObjectData, data::LightingData};
use alexandria::{Id, gpu::GpuAddress};
use buffer::*;

mod buffer;

mod add;
mod get;
mod new;
mod reserve;
mod reset;

/// The render data that exists in two copies for each frame, so that one copy can be used for
/// rendering while the other is being updated
pub(in crate::render::data) struct DoubledRenderData {
    /// The camera data for the current frame
    camera: CameraRenderData,

    /// The data about lighting for the current frame
    lighting: LightingData,

    /// The set of unlit opaque renderable objects in the scene
    ///
    /// These renderables are rendered in a single pass, and do not require any lighting
    /// calculations or transparency
    unlit_opaque_renderables: Vec<(Id<Material>, Id<Mesh>, GpuAddress<ObjectData>)>,

    /// The set of lit opaque renderable objects in the scene
    ///
    /// These renderables are rendered in a single pass, and do not transparency but do use
    /// lighting
    lit_opaque_renderables: Vec<(Id<Material>, Id<Mesh>, GpuAddress<ObjectData>)>,

    /// The buffer for object data
    object_buffer: DataBuffer<ObjectData>,
}
