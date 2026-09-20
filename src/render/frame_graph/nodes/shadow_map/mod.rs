use std::marker::PhantomData;

mod light;

mod create_fixed_objects;
mod create_per_frame_objects;
mod debug;
mod execute;
mod new;
mod usages;

pub(crate) use light::ShadowMapLight;

/// A node that renders shadow maps for lights
pub(in crate::render::frame_graph) struct ShadowMapNode<L: ShadowMapLight> {
    _phantom: PhantomData<L>,
}

#[repr(C)]
struct PushConstants {
    light_index: u32,
    object_index: u32,
}
