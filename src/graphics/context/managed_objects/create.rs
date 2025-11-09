use crate::{
    Result,
    graphics::{
        Camera, CameraInner, CameraProjection, Material, MaterialInner, Shader, ShaderSource,
        context::ManagedGraphicsObjects,
    },
    math::{Color3f, Vector2u},
};
use std::{cell::RefCell, num::NonZeroU32, rc::Rc};
use win32::d3d11::ID3D11Device;

impl ManagedGraphicsObjects {
    /// Create a new [`Camera`]
    pub fn create_camera(
        &mut self,
        projection: CameraProjection,
        screen_size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<Camera> {
        let camera = Rc::new(RefCell::new(CameraInner::new(
            projection,
            screen_size,
            device,
        )?));

        self.cameras.borrow_mut().push(camera.clone());

        Ok(Camera::new(self.cameras.clone(), camera))
    }

    /// Cretae a new [`Shader`]
    pub fn create_shader(
        &mut self,
        source: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Rc<Shader>> {
        let shader = Shader::new(self.next_shader_id, source, device)?;
        self.next_shader_id = unsafe { NonZeroU32::new_unchecked(self.next_shader_id.get() + 1) };
        Ok(shader)
    }

    /// Create a new opaque [`Material`]
    pub fn create_opaque_material(
        &mut self,
        shader: Rc<Shader>,
        color: Color3f,
        device: &ID3D11Device,
    ) -> Result<Material> {
        let material = MaterialInner::new(shader, color, device)?;

        let mut materials = self.opaque_materials.borrow_mut();
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

        Ok(Material::new(self.opaque_materials.clone(), material))
    }
}
