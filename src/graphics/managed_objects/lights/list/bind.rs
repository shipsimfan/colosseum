use crate::{
    Result,
    graphics::managed_objects::lights::{LightList, LightType},
};
use win32::d3d11::{ID3D11Device, ID3D11DeviceContext};

impl<T: LightType> LightList<T> {
    /// Bind this light list to the render pipeline, returning the new number of active lights if
    /// the amount changed
    pub fn bind(
        &mut self,
        device: &ID3D11Device,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<Option<u32>> {
        // Check if the list changed
        let changed_length = self.arena.len() != self.last_len;
        let dirty = if changed_length {
            true
        } else {
            // Check if any light changed
            let mut dirty = false;
            for light in &mut self.arena {
                if light.update() {
                    dirty = true;
                }
            }
            dirty
        };

        // Resize the buffer if needed
        if dirty {
            self.buffer.map(
                self.arena.len(),
                self.arena.iter().map(T::to_gpu),
                device,
                device_context,
            )?;
        }

        // Update last length
        self.last_len = self.arena.len();

        // Bind the buffer
        self.buffer.bind(device_context);

        Ok(if changed_length {
            Some(self.arena.len() as _)
        } else {
            None
        })
    }
}
