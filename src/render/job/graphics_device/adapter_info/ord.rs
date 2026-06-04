use crate::render::job::graphics_device::VulkanAdapterInfo;
use alexandria::gpu::VulkanFormat;
use std::cmp::Ordering;

impl<'instance> PartialOrd for VulkanAdapterInfo<'instance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'instance> Ord for VulkanAdapterInfo<'instance> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.adapter.cmp(&other.adapter) {
            Ordering::Equal => {
                let a = format_score(self.swapchain_format);
                let b = format_score(other.swapchain_format);
                a.cmp(&b)
            }
            other => other,
        }
    }
}

/// Get the score of a Vulkan format for comparison purposes. Lower is better. The exact values are
/// arbitrary, but they should reflect the relative desirability of the formats.
fn format_score(format: VulkanFormat) -> u32 {
    match format {
        VulkanFormat::B8G8R8A8UNorm => 0,
        VulkanFormat::R8G8B8A8UNorm => 1,
        _ => 2,
    }
}
