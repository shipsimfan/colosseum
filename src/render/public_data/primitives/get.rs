use crate::render::{Mesh, Primitives};
use alexandria::Id;

impl Primitives {
    /// Get the ID of the quad primitive
    pub fn quad(&self) -> Id<Mesh> {
        self.quad
    }

    /// Get the ID of the plane primitive
    pub fn plane(&self) -> Id<Mesh> {
        self.plane
    }

    /// Get the ID of the cube primitive
    pub fn cube(&self) -> Id<Mesh> {
        self.cube
    }

    /// Get the ID of the sphere primitive
    pub fn sphere(&self) -> Id<Mesh> {
        self.sphere
    }

    /// Get the ID of the cylinder primitive
    pub fn cylinder(&self) -> Id<Mesh> {
        self.cylinder
    }
}
