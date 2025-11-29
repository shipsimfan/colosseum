use crate::{
    graphics::{
        CameraInner, Material, MaterialInner, Shader,
        context::{Lights, ManagedGraphicsObjects},
    },
    math::Color3f,
};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

impl ManagedGraphicsObjects {
    /// Get the default lit shader
    pub fn default_lit_shader(&self) -> Shader {
        self.default_lit_shader.clone()
    }

    /// Get the default unlit shader
    pub fn default_unlit_shader(&self) -> Shader {
        self.default_unlit_shader.clone()
    }

    /// Get the default lit material
    pub fn default_lit_material(&self) -> Material {
        self.default_lit_material.clone()
    }

    /// Get the default unlit material
    pub fn default_unlit_material(&self) -> Material {
        self.default_unlit_material.clone()
    }

    /// Get the ambient light color
    pub fn ambient_color(&self) -> Color3f {
        self.lights.ambient_color()
    }

    /// Get the intensity of the ambient light
    pub fn ambient_intensity(&self) -> f32 {
        self.lights.ambient_intensity()
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

    /// Get the light manager mutably
    pub(in crate::graphics::context) fn lights(&mut self) -> &mut Lights {
        &mut self.lights
    }
}
