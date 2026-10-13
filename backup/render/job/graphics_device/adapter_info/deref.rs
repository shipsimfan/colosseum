use crate::render::job::graphics_device::VulkanAdapterInfo;
use alexandria::gpu::VulkanAdapter;
use std::ops::Deref;

impl<'instance> Deref for VulkanAdapterInfo<'instance> {
    type Target = VulkanAdapter<'instance>;

    fn deref(&self) -> &Self::Target {
        &self.adapter
    }
}
