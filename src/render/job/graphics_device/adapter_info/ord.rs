use crate::render::job::graphics_device::VulkanAdapterInfo;
use alexandria::gpu::{VulkanAdapterType, VulkanFormat};
use std::cmp::Ordering;

impl<'instance> PartialOrd for VulkanAdapterInfo<'instance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'instance> Ord for VulkanAdapterInfo<'instance> {
    fn cmp(&self, other: &Self) -> Ordering {
        match type_score(self.r#type).cmp(&type_score(other.r#type)) {
            Ordering::Equal => {}
            other => return other,
        }

        match self.device_local_vram.cmp(&other.device_local_vram) {
            Ordering::Equal => {}
            other => return other,
        }

        match format_score(self.swapchain_format).cmp(&format_score(other.swapchain_format)) {
            Ordering::Equal => {}
            other => return other,
        }

        match self.name.cmp(&other.name) {
            Ordering::Equal => {}
            other => return other,
        }

        self.uuid.cmp(&other.uuid)
    }
}

/// Get the score of a Vulkan adapter type for comparison purposes. Lower is better. The exact
/// values are arbitrary, but they should reflect the relative desirability of the adapter types.
fn type_score(r#type: VulkanAdapterType) -> u8 {
    match r#type {
        VulkanAdapterType::DiscreteGPU => 0,
        VulkanAdapterType::IntegratedGPU => 1,
        VulkanAdapterType::VirtualGPU => 2,
        VulkanAdapterType::CPU => 3,
        _ => 4,
    }
}

/// Get the score of a Vulkan format for comparison purposes. Lower is better. The exact values are
/// arbitrary, but they should reflect the relative desirability of the formats.
fn format_score(format: VulkanFormat) -> u8 {
    match format {
        VulkanFormat::B8G8R8A8UNorm => 0,
        VulkanFormat::R8G8B8A8UNorm => 1,
        _ => 2,
    }
}
