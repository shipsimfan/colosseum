use std::{cell::RefCell, rc::Rc};
use win32::{
    ComPtr,
    d3d11::{ID3D11Buffer, ID3D11ShaderResourceView},
};

mod bind;
mod light_type;
mod new;
mod push;

pub(in crate::graphics) use light_type::LightType;

/// A managed list of lights
pub(in crate::graphics::context::managed_objects::lights) struct LightList<T: LightType> {
    /// The list of lights shared with the lights themselves, including a dirty marker for the list
    shared_list: Rc<RefCell<(Vec<Rc<RefCell<T>>>, bool)>>,

    /// The slot to bind the GPU buffer to
    buffer_slot: u32,

    /// The cache of the GPU buffer
    buffer_cache: Vec<T::GPU>,

    /// The number of lights the buffer can currently handle
    buffer_capacity: usize,

    /// The buffer of the light information on the GPU
    buffer: ComPtr<ID3D11Buffer>,

    /// The view of the light information for shaders
    buffer_view: ComPtr<ID3D11ShaderResourceView>,
}
