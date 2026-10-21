use crate::render::frame_graph::{FrameGraphResourceState, FrameGraphTransientResourceInfo};
use alexandria::gpu::VulkanFormat;
use std::ffi::CString;

impl FrameGraphTransientResourceInfo {
    /// Create a new [`FrameGraphTransientResourceInfo`]
    pub fn new(name: &'static str, format: VulkanFormat) -> FrameGraphTransientResourceInfo {
        FrameGraphTransientResourceInfo {
            name,
            image_name: [
                create_image_name(name, 0),
                create_image_name(name, 1),
                create_image_name(name, 2),
            ],
            image_view_name: [
                create_image_view_name(name, 0),
                create_image_view_name(name, 1),
                create_image_view_name(name, 2),
            ],
            format,

            state: FrameGraphResourceState::default(),

            is_color: false,
            is_depth: false,
            is_transfer_dst: false,
            is_transfer_src: false,
            is_sampled_image: false,
        }
    }
}

/// Create the image name for a given frame index
fn create_image_name(name: &'static str, index: usize) -> CString {
    CString::new(format!("{} Image {}", name, index)).unwrap()
}

/// Create the image view name for a given frame index
fn create_image_view_name(name: &'static str, index: usize) -> CString {
    CString::new(format!("{} Image View {}", name, index)).unwrap()
}
