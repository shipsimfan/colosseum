use crate::{Result, render::RenderData};

impl RenderData {
    /// Reserve enough space to store all the renderables
    pub fn reserve_renderables(&mut self, num: usize) -> Result<()> {
        self.doubled[self.current_doubled_index].reserve_renderables(
            num,
            &self.device,
            &self.memory_properties,
        )
    }

    /// Reserve enough space to store all the directional lights
    pub fn reserve_directional_lights(&mut self, num: usize) -> Result<()> {
        self.doubled[self.current_doubled_index]
            .lighting_mut()
            .reserve_directional_lights(num, &self.device, &self.memory_properties)
    }

    /// Reserve enough space to store all the point lights
    pub fn reserve_point_lights(&mut self, num: usize) -> Result<()> {
        self.doubled[self.current_doubled_index]
            .lighting_mut()
            .reserve_point_lights(num, &self.device, &self.memory_properties)
    }

    /// Reserve enough space to store all the spot lights
    pub fn reserve_spot_lights(&mut self, num: usize) -> Result<()> {
        self.doubled[self.current_doubled_index]
            .lighting_mut()
            .reserve_spot_lights(num, &self.device, &self.memory_properties)
    }
}
