use crate::{
    Result,
    render::{
        FixedRenderObjects, RenderData, RenderObjects,
        frame_graph::{FrameGraphResourceId, FrameGraphResourceUsage, FrameGraphResources},
    },
};
use alexandria::gpu::{
    VulkanCommandBuffer, VulkanDescriptorPool, VulkanDescriptorSet, VulkanDevice, VulkanFormat,
};
use r#macro::nodes;

mod r#macro;

nodes![
    unsampled: [
        /** Main Render Nodes **/

        /// A node that clears the screen to a solid color
        solid_color_sky::SolidColorSky(SolidColorSkyNode),

        /// A node that renders unlit objects using a forward rendering pipeline
        unlit_forward_render::UnlitForwardRender(UnlitForwardRenderNode),

        /** Post-Processing Nodes **/

        /// A node that changes the render scale of the input image and outputs it to a new image
        render_scale::RenderScale(RenderScaleNode),
    ],

    sampled: [
        /// A node that performs sharpening, gamma correction, and dithering on the input image and
        /// outputs it to a new image
        gamma_correction::GammaCorrection(GammaCorrectionNode),
    ]
];
