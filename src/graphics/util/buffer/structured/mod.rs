use std::marker::PhantomData;

use win32::{
    ComPtr,
    d3d11::{ID3D11Buffer, ID3D11ShaderResourceView},
};

mod bind;
mod map;
mod new;

/// A buffer which contains an array of elements only changed from the CPU
pub(in crate::graphics) struct StructuredBuffer<T: Sized + Copy> {
    /// The buffer itself
    buffer: ComPtr<ID3D11Buffer>,

    /// The present capacity of the GPU buffer
    buffer_capacity: usize,

    /// The view for accessing the structured buffer
    view: ComPtr<ID3D11ShaderResourceView>,

    /// The slot to bind the structured buffer to
    slot: u32,

    /// A marker for the type
    _type: PhantomData<T>,
}
