use alexandria::{PhysicalDevice, PhysicalDeviceType};

/// Searches `physical_devices` for a device named `device_name` or the best available one if it
/// can't be found or isn't provided.
pub(super) fn select_physical_device<'a>(
    device_name: Option<&str>,
    physical_devices: &'a [PhysicalDevice],
) -> Option<&'a PhysicalDevice> {
    let mut best_score = 0;
    let mut best_device = None;
    for physical_device in physical_devices {
        // Check if the device's name matches
        if device_name.is_some() && physical_device.name() == device_name.unwrap() {
            return Some(physical_device);
        }

        // Compare scores
        let score = calculate_score(physical_device);
        if best_device.is_none() || score > best_score {
            best_device = Some(physical_device);
            best_score = score;
        }
    }

    best_device
}

/// Calculates a score for `physical_device` representing how good the device is. Higher numbers
/// are better.
fn calculate_score(physical_device: &PhysicalDevice) -> usize {
    match physical_device.r#type() {
        PhysicalDeviceType::DiscreteGPU => 4,
        PhysicalDeviceType::IntegratedGPU => 3,
        PhysicalDeviceType::VirtualGPU => 2,
        PhysicalDeviceType::CPU => 1,
        _ => 0,
    }
}
