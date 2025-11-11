use crate::graphics::{
    CameraInner, Material, MaterialInner, Shader, context::ManagedGraphicsObjects,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

impl ManagedGraphicsObjects {
    /// Get the default shader
    pub fn default_shader(&self) -> Shader {
        self.default_shader.clone()
    }

    /// Get the default material
    pub fn default_material(&self) -> Material {
        self.default_material.clone()
    }

    /// Get the list of cameras in the system
    pub(in crate::graphics) fn cameras<'a>(&'a self) -> Ref<'a, Vec<Rc<RefCell<CameraInner>>>> {
        self.cameras.borrow()
    }

    /// Get the list of cameras in the system
    pub(in crate::graphics) fn opaque_materials<'a>(
        &'a self,
    ) -> Ref<'a, Vec<Rc<RefCell<MaterialInner>>>> {
        self.opaque_materials.borrow()
    }
}
