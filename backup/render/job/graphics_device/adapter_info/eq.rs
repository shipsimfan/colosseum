use crate::render::job::graphics_device::VulkanAdapterInfo;

impl<'instance> PartialEq for VulkanAdapterInfo<'instance> {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.name == other.name && self.uuid == other.uuid
    }
}

impl<'instance> Eq for VulkanAdapterInfo<'instance> {}
