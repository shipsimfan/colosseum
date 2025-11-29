use crate::{
    Result,
    graphics::{Material, MaterialInner, Shader},
    math::Color3f,
};
use std::{cell::RefCell, rc::Rc};
use win32::d3d11::ID3D11Device;

impl Material {
    /// Create a new [`Material`]
    pub(in crate::graphics) fn new(
        material_list: Rc<RefCell<Vec<Rc<RefCell<MaterialInner>>>>>,
        shader: Shader,
        color: Color3f,
        specular_strength: f32,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let material = MaterialInner::new(shader, color, specular_strength, device)?;

        let mut materials = material_list.borrow_mut();
        let mut insert_idx = None;
        for (i, mat) in materials.iter().enumerate() {
            if mat.borrow().shader().id() > material.shader().id() {
                insert_idx = Some(i);
                break;
            }
        }

        let material = Rc::new(RefCell::new(material));
        match insert_idx {
            Some(idx) => materials.insert(idx, material.clone()),
            None => materials.push(material.clone()),
        }

        drop(materials);

        Ok(Material {
            material_list,
            material,
        })
    }
}
