use win32::{ComPtr, d3d11::ID3D11Buffer};

mod bind;
mod deref;
mod new;

/// A buffer which contains an array of elements only changed from the CPU
pub(in crate::graphics) struct InstanceBuffer<T: Sized + Copy> {
    /// The buffer itself
    buffer: ComPtr<ID3D11Buffer>,

    /// The content contained in the instance buffer
    content: Box<[T]>,

    /// Have the contents changed since the last bind?
    dirty: bool,

    /// The slot to bind the constant buffer to
    slot: u32,
}
