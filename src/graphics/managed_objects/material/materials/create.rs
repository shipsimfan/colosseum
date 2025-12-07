use crate::{
    Result,
    graphics::{Material, MaterialHandle, Materials},
    math::Color3f,
};

impl Materials {
    /// Create a new lit [`Material`]
    pub fn create_lit(&mut self, color: Color3f, specular_strength: f32) -> Result<MaterialHandle> {
        let id = self.next_material_id;
        self.next_material_id = self.next_material_id.checked_add(1).unwrap();

        Ok(self.arena.insert(Material::new(
            id,
            self.lit_shader.clone(),
            color,
            specular_strength,
            &self.device,
        )?))
    }

    /// Create a new unlit [`Material`]
    pub fn create_unlit(
        &mut self,
        color: Color3f,
        specular_strength: f32,
    ) -> Result<MaterialHandle> {
        let id = self.next_material_id;
        self.next_material_id = self.next_material_id.checked_add(1).unwrap();

        Ok(self.arena.insert(Material::new(
            id,
            self.unlit_shader.clone(),
            color,
            specular_strength,
            &self.device,
        )?))
    }
}
