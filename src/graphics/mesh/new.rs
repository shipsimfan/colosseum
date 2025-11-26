use crate::{
    Error, Result,
    graphics::{Mesh, MeshInner, Vertex},
};
use std::{borrow::Cow, rc::Rc};

impl Mesh {
    /// Create a new [`Mesh`]
    pub fn new<V: Into<Cow<'static, [Vertex]>>, I: Into<Cow<'static, [u32]>>>(
        vertices: V,
        indices: I,
    ) -> Result<Self> {
        let vertices = vertices.into();
        let indices = indices.into();

        // Make sure the length of indices makes sense
        if indices.len() % 3 != 0 {
            return Err(Error::new(format!(
                "the mesh indices len is {} which isn't a multiple of 3",
                indices.len()
            )));
        }

        // Make sure indices are in bounds
        for index in indices.iter() {
            if *index >= vertices.len() as _ {
                return Err(Error::new(format!(
                    "mesh index is {} but there are only {} vertices",
                    index,
                    vertices.len()
                )));
            }
        }

        Ok(unsafe { Mesh::new_unchecked(vertices, indices) })
    }

    /// Create a new [`Mesh`] without validating the values
    pub unsafe fn new_unchecked<V: Into<Cow<'static, [Vertex]>>, I: Into<Cow<'static, [u32]>>>(
        vertices: V,
        indices: I,
    ) -> Self {
        let inner = Rc::new(MeshInner::new(vertices.into(), indices.into()));
        Mesh { inner }
    }
}
