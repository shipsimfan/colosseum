use crate::render::{FixedRenderObjects, GpuTransferQueue, Material, Mesh, Shader, ShaderId};
use alexandria::{Id, SlotMap, gpu::VulkanDevice};
use std::sync::Arc;

mod allocator;

mod apply;
mod complete;
mod create;
mod get;
mod new;
mod remove;

pub(crate) use allocator::*;

/// The representations of the render objects in the update phase
pub(crate) struct UpdateRenderObjects {
    /// The device to use to create render objects
    device: VulkanDevice,

    /// The fixed render objects
    fixed_render_objects: Arc<FixedRenderObjects>,

    /// The queue for transfering data to the GPU
    transfer_queue: GpuTransferQueue,

    /// The allocator to use for meshes
    mesh_allocator: GpuAllocator,

    /// The meshes that have been registered
    meshes: SlotMap<(Mesh, GpuAllocatedMemory)>,

    /// The unlit shaders that have been registered
    ///
    /// These shaders are run in a forward pass without lighting information
    unlit_shaders: SlotMap<Arc<Shader>>,

    /// The ID of the default unlit shader to use when no other shader is specified
    default_unlit_shader: ShaderId,

    /// The unlit opaque materials that have been registered
    ///
    /// These materials are used in a forward pass without lighting information or transparency
    unlit_opaque_materials: SlotMap<Material>,

    /// The ID of the quad primitive mesh
    quad: Id<Mesh>,

    /// The ID of the plane primitive mesh
    plane: Id<Mesh>,

    /// The ID of the cube primitive mesh
    cube: Id<Mesh>,

    /// The ID of the sphere primitive mesh
    sphere: Id<Mesh>,

    /// The ID of the cylinder primitive mesh
    cylinder: Id<Mesh>,
}
