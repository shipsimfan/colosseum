use crate::{
    Result,
    render::{
        DeviceDataBuffer, FixedRenderObjects, PerFrameObjectBuilder, RenderData, RenderObjects,
        frame_graph::{FrameGraphResourceId, FrameGraphResourceUsage, FrameGraphResources},
    },
};
use alexandria::gpu::{
    VulkanAdapterMemoryProperties, VulkanCommandBuffer, VulkanDevice, VulkanFormat,
};
use r#macro::nodes;

mod r#macro;

nodes![
    simple: [
        /// A node that clears the screen to a solid color
        solid_color_sky::SolidColorSky(SolidColorSkyNode),
    ],

    data_buffer:[
        /// A node that renders unlit objects using a forward rendering pipeline
        unlit_forward_render::UnlitForwardRender(UnlitForwardRenderNode),

        /// A node that renders lit objects using a forward rendering pipeline
        lit_forward_render::LitForwardRender(LitForwardRenderNode),

    ],

    post_process: [
        /// A node that changes the render scale of the input image and outputs it to a new image
        render_scale::RenderScale(RenderScaleNode),

        /// A node that performs color correction, tone mapping, gamma correction, and color
        /// grading
        tone_map::ToneMap(ToneMapNode),

        /// A node that performs sharpening and dithering before quantizing an input image
        quantization::Quantization(QuantizationNode),

        /// A node that performs FXAA anti-aliasing on an input image
        fxaa::Fxaa(FxaaNode),
    ]
];
