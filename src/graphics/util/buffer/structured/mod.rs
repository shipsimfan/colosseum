use std::marker::PhantomData;
use win32::{ComPtr, d3d11::ID3D11Buffer};

/// A buffer which contains an array of elements only changed from the CPU
pub struct StructuredBuffer<T: Sized> {
    /// The buffer itself
    buffer: ComPtr<ID3D11Buffer>,

    /// A placeholder for the type
    _type: PhantomData<T>,
}
