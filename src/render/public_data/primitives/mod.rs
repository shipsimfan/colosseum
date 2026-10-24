use crate::render::Mesh;
use alexandria::Id;

mod get;
mod new;

/// The primitive meshes that are available in the engine
pub struct Primitives {
    /// The ID of the quad primitive
    quad: Id<Mesh>,

    /// The ID of the plane primitive
    plane: Id<Mesh>,

    /// The ID of the cube primitive
    cube: Id<Mesh>,

    /// The ID of the sphere primitive
    sphere: Id<Mesh>,

    /// The ID of the cylinder primitive
    cylinder: Id<Mesh>,
}
