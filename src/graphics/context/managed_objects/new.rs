use crate::{
    Result,
    graphics::{Material, Shader, context::ManagedGraphicsObjects},
    math::Color3f,
};
use std::{cell::RefCell, num::NonZeroU32, rc::Rc};
use win32::d3d11::ID3D11Device;

impl ManagedGraphicsObjects {
    /// Creates a new empty set of [`ManagedGraphicsObjects`]
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        // Create default shader
        let default_shader = Shader::create_default(NonZeroU32::new(1).unwrap(), device)?;

        // Create default material
        let opaque_materials = Rc::new(RefCell::new(Vec::new()));
        let default_material = Material::new(
            opaque_materials.clone(),
            default_shader.clone(),
            Color3f::WHITE,
            device,
        )?;

        // Create objects container
        Ok(ManagedGraphicsObjects {
            cameras: Rc::new(RefCell::new(Vec::new())),
            opaque_materials,
            default_shader,
            default_material,
            next_shader_id: NonZeroU32::new(2).unwrap(),
        })
    }
}
