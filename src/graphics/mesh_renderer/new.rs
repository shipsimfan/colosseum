use crate::{
    Result,
    graphics::{Material, Mesh, MeshRenderer, MeshRendererInner},
};
use std::{cell::RefCell, rc::Rc};
use win32::d3d11::ID3D11Device;

impl MeshRenderer {
    /// Create a new [`MeshRenderer`]
    pub(in crate::graphics) fn new(
        material: Material,
        mesh: Mesh,
        max_instances: usize,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let mesh_renderer = Rc::new(RefCell::new(MeshRendererInner::new(
            material.clone(),
            mesh,
            max_instances,
            device,
        )?));

        material
            .borrow_mut()
            .push_mesh_renderer(mesh_renderer.clone());

        Ok(MeshRenderer { mesh_renderer })
    }
}
