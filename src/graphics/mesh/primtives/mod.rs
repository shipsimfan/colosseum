use std::marker::PhantomData;

mod quad;
mod cube;

/// A struct provides functions to produce primitive meshes
pub struct MeshPrimitives {
    /// A field to prevent this struct from being created elsewhere
    _priv: PhantomData<()>,
}
