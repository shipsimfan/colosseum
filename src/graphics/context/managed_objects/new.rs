use crate::{
    Result,
    graphics::{
        Material, Shader,
        context::{Lights, ManagedGraphicsObjects},
    },
    math::Color3f,
};
use std::{
    cell::RefCell,
    num::{NonZero, NonZeroU32},
    rc::Rc,
};
use win32::d3d11::ID3D11Device;

const DEFAULT_SPECULAR_STRENGTH: f32 = 0.5;

impl ManagedGraphicsObjects {
    /// Creates a new empty set of [`ManagedGraphicsObjects`]
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        // Create default shaders
        let default_lit_shader = Shader::create_default_lit(NonZero::new(1).unwrap(), device)?;
        let default_unlit_shader =
            Shader::create_default_unlit(NonZeroU32::new(2).unwrap(), device)?;

        // Create default materials
        let opaque_materials = Rc::new(RefCell::new(Vec::new()));
        let default_lit_material = Material::new(
            opaque_materials.clone(),
            default_lit_shader.clone(),
            Color3f::WHITE,
            DEFAULT_SPECULAR_STRENGTH,
            device,
        )?;
        let default_unlit_material = Material::new(
            opaque_materials.clone(),
            default_unlit_shader.clone(),
            Color3f::WHITE,
            DEFAULT_SPECULAR_STRENGTH,
            device,
        )?;

        // Create lights
        let lights = Lights::new(device)?;

        // Create objects container
        Ok(ManagedGraphicsObjects {
            cameras: Rc::new(RefCell::new(Vec::new())),
            opaque_materials,
            lights,
            default_lit_shader,
            default_unlit_shader,
            default_lit_material,
            default_unlit_material,
            next_shader_id: NonZeroU32::new(3).unwrap(),
        })
    }
}
