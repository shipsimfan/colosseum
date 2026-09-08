use crate::{
    Error, Result,
    render::{DeviceBufferDescriptorSet, DeviceDataBuffer},
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanBufferUsageFlag, VulkanBufferUsageFlags,
    VulkanDescriptorBufferInfo, VulkanDescriptorSet, VulkanDescriptorType, VulkanDevice,
    VulkanMemoryPropertyFlag, VulkanSharingMode, VulkanWriteDescriptorSet,
};
use std::ffi::CString;

impl DeviceDataBuffer {
    /// Create a new [`DeviceDataBuffer`]
    pub(in crate::render::frame_graph::resources::buffer) fn new<T>(
        name: String,
        capacity: usize,
        usage: VulkanBufferUsageFlags,
        descriptor_type: VulkanDescriptorType,
        descriptor_sets: Vec<DeviceBufferDescriptorSet>,

        created_descriptor_sets: &[VulkanDescriptorSet],
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<DeviceDataBuffer> {
        let name = CString::new(name).unwrap();
        DeviceDataBuffer::new_inner::<T>(
            name,
            capacity,
            usage,
            descriptor_type,
            descriptor_sets,
            created_descriptor_sets,
            device,
            memory_properties,
        )
    }

    /// Create a new [`DeviceDataBuffer`]
    pub(in crate::render::frame_graph::resources::buffer) fn new_inner<T>(
        #[cfg_attr(not(debug_assertions), allow(unused_variables))] name: CString,
        capacity: usize,
        usage: VulkanBufferUsageFlags,
        descriptor_type: VulkanDescriptorType,
        descriptor_sets: Vec<DeviceBufferDescriptorSet>,

        created_descriptor_sets: &[VulkanDescriptorSet],
        device: &VulkanDevice,
        memory_properties: &VulkanAdapterMemoryProperties,
    ) -> Result<DeviceDataBuffer> {
        let size = (capacity * std::mem::size_of::<T>()) as u64;

        // Create the buffer
        let usage = VulkanBufferUsageFlag::TransferDst | usage;
        let mut buffer = device
            .create_buffer(0, size, usage, VulkanSharingMode::Exclusive, &[])
            .map_err(Error::new_inner)?;

        // Allocate memory for the buffer
        let memory_requirements = buffer.get_memory_requirements();
        let memory_type_index = memory_properties
            .find_memory_type(
                memory_requirements.memory_type_bits(),
                VulkanMemoryPropertyFlag::DeviceLocal,
            )
            .ok_or(Error::new("cannot find memory for a device buffer"))?;
        let memory = device
            .allocate_memory(memory_requirements.size(), memory_type_index)
            .map_err(Error::new_inner)?;

        // Bind the buffer to the allocated memory
        buffer.bind_memory(&memory, 0).map_err(Error::new_inner)?;

        // Bind the buffer to a descriptor set
        for descriptor_set in &descriptor_sets {
            device.update_descriptor_sets(
                &[VulkanWriteDescriptorSet::new(
                    &created_descriptor_sets[descriptor_set.descriptor_set],
                    descriptor_set.binding,
                    0,
                    descriptor_type,
                    &[],
                    &[VulkanDescriptorBufferInfo::new(&buffer, 0, size)],
                )],
                &[],
            );
        }

        Ok(DeviceDataBuffer {
            name,
            capacity: size as usize,
            buffer,
            memory,
            usage,
            descriptor_sets,
            descriptor_type,
        })
    }
}
