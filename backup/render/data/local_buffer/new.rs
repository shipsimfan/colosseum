use crate::{Error, Result, render::LocalDataBuffer};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferUsageFlag, VulkanDevice, VulkanMemoryPropertyFlag,
    VulkanSharingMode,
};
use std::{ffi::CString, rc::Rc};

impl<T> LocalDataBuffer<T> {
    /// Create a new [`LocalDataBuffer`]
    pub(in crate::render::data) fn new(
        name: String,
        capacity: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<LocalDataBuffer<T>> {
        let memory_name = Rc::new(CString::new(format!("{} Memory", name)).unwrap());
        let buffer_name = Rc::new(CString::new(name).unwrap());
        LocalDataBuffer::new_inner(
            buffer_name,
            memory_name,
            capacity,
            device,
            memory_properties,
        )
    }

    /// Create a new [`LocalDataBuffer`]
    pub(in crate::render::data::local_buffer) fn new_inner(
        #[cfg_attr(not(debug_assertions), allow(unused_variables))] buffer_name: Rc<CString>,
        #[cfg_attr(not(debug_assertions), allow(unused_variables))] memory_name: Rc<CString>,
        capacity: usize,
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<LocalDataBuffer<T>> {
        let size = (capacity * std::mem::size_of::<T>()) as u64;

        // Create the buffer
        let mut buffer = device
            .create_buffer(
                0,
                size,
                VulkanBufferUsageFlag::TransferSrc,
                VulkanSharingMode::Exclusive,
                &[],
            )
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut buffer, &buffer_name)
            .map_err(Error::new_inner)?;

        // Allocate memory for the buffer
        let memory_requirements = buffer.get_memory_requirements();
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::HostVisible | VulkanMemoryPropertyFlag::HostCoherent,
            )
            .ok_or(Error::new("cannot find memory for a buffer"))?;
        let mut memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut memory, &memory_name)
            .map_err(Error::new_inner)?;

        // Bind the buffer to the allocated memory
        buffer.bind_memory(&memory, 0).map_err(Error::new_inner)?;

        // Map the buffer memory to a pointer
        let memory = memory
            .map(0, size, 0)
            .map_err(|(error, _)| Error::new_inner(error))?;

        Ok(LocalDataBuffer {
            buffer_name,
            memory_name,
            capacity,
            count: 0,

            buffer,
            memory,
        })
    }
}
