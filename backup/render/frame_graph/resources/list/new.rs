use crate::render::frame_graph::resources::FrameGraphResourceList;
use std::ffi::CString;

impl FrameGraphResourceList {
    /// Create a new [`FrameGraphResourceList`]
    pub fn new(name: &'static str, index: usize) -> FrameGraphResourceList {
        let name = CString::new(format!("{} Transient Resource Memory {}", name, index)).unwrap();

        FrameGraphResourceList {
            name,
            resources: Vec::new(),
            memory: None,
        }
    }
}
