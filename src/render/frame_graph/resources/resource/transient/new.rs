use crate::render::frame_graph::{FrameGraphResourceState, FrameGraphTransientResourceInfo};
use alexandria::gpu::{VulkanAttachmentLoadOp, VulkanFormat};
use std::ffi::CString;

impl<ExtraData> FrameGraphTransientResourceInfo<ExtraData> {
    /// Create a new [`FrameGraphTransientResourceInfo`]
    pub fn new(
        name: &'static str,
        format: VulkanFormat,
        initial_load_op: VulkanAttachmentLoadOp,
        extra_data: ExtraData,
    ) -> FrameGraphTransientResourceInfo<ExtraData> {
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
            load_op: initial_load_op,

            is_color: false,
            is_depth: false,
            is_transfer_dst: false,
            is_transfer_src: false,
            is_sampled_image: false,

            descriptor_sets: Vec::new(),

            extra_data,
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
