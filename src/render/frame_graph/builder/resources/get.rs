use crate::render::frame_graph::{
    FrameGraphResourceId,
    builder::{FrameGraphBuilderResource, FrameGraphBuilderResources},
};

impl FrameGraphBuilderResources {
    pub fn get<'a>(&'a mut self, id: FrameGraphResourceId) -> FrameGraphBuilderResource<'a> {
        if id.is_external() {
            FrameGraphBuilderResource::External(&mut self.external_resources[id.index()])
        } else if id.is_transient_native_scale() {
            FrameGraphBuilderResource::Transient(&mut self.native_scale_resources[id.index()])
        } else if id.is_transient_render_scale() {
            FrameGraphBuilderResource::Transient(&mut self.render_scale_resources[id.index()])
        } else {
            panic!("unsupported resource type for color attachment");
        }
    }
}
