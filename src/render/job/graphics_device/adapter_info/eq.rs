use crate::render::job::graphics_device::VulkanAdapterInfo;

impl<'instance> PartialEq for VulkanAdapterInfo<'instance> {
    fn eq(&self, other: &Self) -> bool {
        self.adapter == other.adapter
    }
}

impl<'instance> Eq for VulkanAdapterInfo<'instance> {}
