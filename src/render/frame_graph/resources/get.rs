use crate::render::{
    FrameIndex,
    frame_graph::{
        FrameGraphResource, FrameGraphResourceId, FrameGraphResources, FrameGraphShadowMapBuffer,
    },
};
use alexandria::gpu::VulkanDescriptorSet;

impl FrameGraphResources {
    /// Get a resource based on its id
    pub fn get<'a>(
        &'a self,
        id: FrameGraphResourceId,
        frame_index: &FrameIndex,
    ) -> FrameGraphResource<'a> {
        if id.is_external() {
            (&self.external_resources[id.index()]).into()
        } else if id.is_transient_native_scale() {
            let (info, image_view, size) = self.native_scale.get(id.index(), **frame_index);
            FrameGraphResource::Transient {
                info,
                image_view,
                size,
            }
        } else if id.is_transient_render_scale() {
            let (info, image_view, size) = self.render_scale.get(id.index(), **frame_index);
            FrameGraphResource::Transient {
                info,
                image_view,
                size,
            }
        } else if id.is_shadow_map() {
            let (info, image_view, size, array_layers) =
                self.shadow_maps[id.index()].get_full(**frame_index);
            FrameGraphResource::ShadowMap {
                info,
                image_view,
                size,
                array_layers,
            }
        } else {
            unreachable!("unknown frame graph resource type")
        }
    }

    /// Get a reference to a shadow map
    pub fn get_shadow_map(&self, id: FrameGraphResourceId) -> &FrameGraphShadowMapBuffer {
        assert!(id.is_shadow_map(), "id must be a shadow map");
        &self.shadow_maps[id.index()]
    }

    /// Get a descriptor set based on its id
    pub fn descriptor_set(&self, id: usize, frame_index: &FrameIndex) -> &VulkanDescriptorSet {
        &self.descriptor_sets[id].0[**frame_index]
    }
}
