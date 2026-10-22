use crate::{
    Result,
    render::{DescriptorSetLayoutInfo, DescriptorSetLayouts, GraphicsDevice},
};
use colosseum_core::new_error;
use std::any::TypeId;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl DescriptorSetLayouts {
    /// Create a new descriptor set layout
    pub fn create<T: DescriptorSetLayoutInfo>(
        &mut self,
        name: &str,
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
            .set_object_name(&mut handle, &CString::new(name).unwrap())
            .map_err(|error| {
                new_error!(
                    "unable to set descriptor set layout name for \"{}\" - {}",
                    name,
                    error
                )
            })?;

        self.layouts.push((TypeId::of::<T>(), handle));
        Ok(())
    }
}
