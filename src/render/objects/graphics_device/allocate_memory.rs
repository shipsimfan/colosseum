#[cfg(debug_assertions)]
use std::{ffi::CString, str::FromStr};

use crate::{Result, render::GraphicsDevice};
use alexandria::gpu::{VulkanDeviceMemory, VulkanMemoryPropertyFlags, VulkanMemoryRequirements};
use colosseum_core::new_error;

impl GraphicsDevice {
    /// Allocate a block of memory on the graphics device
    pub fn allocate_memory<P: Into<VulkanMemoryPropertyFlags>>(
        &self,
        requirements: &VulkanMemoryRequirements,
        properties: P,
        name: &str,
    ) -> Result<VulkanDeviceMemory> {
        let properties = properties.into();

        let memory_type_index = self
            .memory_properties
            .find_memory_type(requirements.memory_type_bits(), properties)
            .ok_or_else(|| new_error!("unable to find a memory type for \"{}\"", name))?;

        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut memory = self
            .device
            .allocate_memory(requirements.size(), memory_type_index)
            .map_err(|error| {
                new_error!("unable to allocate memory for \"{}\" - {}", name, error)
            })?;

        #[cfg(debug_assertions)]
        self.set_object_name(&mut memory, &CString::from_str(name).unwrap())
            .map_err(|error| new_error!("unable to set memory name to \"{}\" - {}", name, error))?;

        Ok(memory)
    }
}
