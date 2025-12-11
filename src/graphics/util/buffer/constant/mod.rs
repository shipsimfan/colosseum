use win32::{ComPtr, d3d11::ID3D11Buffer};

mod bind;
mod deref;
mod new;
mod slots;

/// A buffer which contains a single element and is only changed from the CPU
pub(in crate::graphics) struct ConstantBuffer<T: Sized + Copy> {
    /// The buffer itself
    buffer: ComPtr<ID3D11Buffer>,

    /// The content contained in the constant buffer
    content: T,

    /// Have the contents changed since the last bind?
    dirty: bool,

    /// The slot to bind the constant buffer to
    slot: u32,
}
