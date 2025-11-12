use crate::{
    Error, Result,
    graphics::{Mesh, MeshInner, Vertex},
};
use std::rc::Rc;
use win32::d3d11::ID3D11Device;

impl Mesh {
    /// Create a new [`Mesh`]
    pub(in crate::graphics) fn new(
        vertices: &[Vertex],
        indices: &[u32],
        device: &ID3D11Device,
    ) -> Result<Self> {
        if indices.len() % 3 != 0 {
            return Err(Error::new(format!(
                "the mesh indices len is {} which isn't a multiple of 3",
                indices.len()
            )));
        }

        for index in indices {
            if *index >= vertices.len() as _ {
                return Err(Error::new(format!(
                    "mesh index is {} but there are only {} vertices",
                    index,
                    vertices.len()
                )));
            }
        }

        unsafe { Mesh::new_unchecked(vertices, indices, device) }
    }

    /// Create a new [`Mesh`] without validating the values
    pub(in crate::graphics) unsafe fn new_unchecked(
        vertices: &[Vertex],
        indices: &[u32],
        device: &ID3D11Device,
    ) -> Result<Self> {
        let inner = Rc::new(MeshInner::new(vertices, indices, device)?);
        Ok(Mesh { inner })
    }
}
