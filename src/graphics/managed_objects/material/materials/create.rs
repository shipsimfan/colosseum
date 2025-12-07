use crate::{
    Result,
    graphics::{Material, MaterialHandle, Materials, Shader},
    math::Color3f,
};

impl Materials {
    /// Create a new [`Material`]
    pub fn create(
        &mut self,
        shader: Shader,
        color: Color3f,
        specular_strength: f32,
    ) -> Result<MaterialHandle> {
        let id = self.next_material_id;
        self.next_material_id = self.next_material_id.checked_add(1).unwrap();

        Ok(self.arena.insert(Material::new(
            id,
            shader,
            color,
            specular_strength,
            &self.device,
        )?))
    }
}
