use crate::{
    Result,
    render::{DescriptorSetLayout, DescriptorSetLayoutInfo, DescriptorSetLayouts, GraphicsDevice},
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl DescriptorSetLayouts {
    /// Create a new descriptor set layout
    pub fn create<T: DescriptorSetLayoutInfo>(
        &mut self,
        name: &'static str,
        device: &GraphicsDevice,
    ) -> Result<()> {
        let mut bindings = Vec::with_capacity(T::BINDINGS.len());
        for (i, binding) in T::BINDINGS.iter().enumerate() {
            bindings.push(binding.to_vulkan(i as _));
        }

        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut handle = device
            .create_descriptor_set_layout(0, &bindings)
            .map_err(|error| new_error!("unable to create \"{}\" - {}", name, error))?;

        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut handle,
                &CString::new(format!("{} Descriptor Set Layout", name)).unwrap(),
            )
            .map_err(|error| {
                new_error!(
                    "unable to set descriptor set layout name for \"{}\" - {}",
                    name,
                    error
                )
            })?;

        self.layouts
            .push(DescriptorSetLayout::new::<T>(name, handle));
        Ok(())
    }
}
