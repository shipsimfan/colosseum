use crate::graphics::{Mesh, MeshInner};
use std::ops::Deref;

impl Deref for Mesh {
    type Target = MeshInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
