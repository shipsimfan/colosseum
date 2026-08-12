macro_rules! nodes {
    [$(
        $(#[$meta: meta])*
        $module: ident::$name: ident($type: ident),
    )*] => {
        $(
            mod $module;

            pub(in crate::render::frame_graph) use $module::$type;
        )*

        /// A single node in the frame graph
        #[derive(Debug)]
        #[allow(private_interfaces)]
        pub(in crate::render) enum FrameGraphNode {$(
            $(#[$meta])*
            $name($type),
        )*}

        impl FrameGraphNode {
            /// Execute this node, performing the rendering operations associated with it
            pub(in crate::render::frame_graph) fn execute(
                &self,
                render_data: &RenderData,
                swapchain_size: Vector2u,
                cmd_buffer: &mut VulkanCommandBuffer,

                render_objects: &RenderObjects,
            ) {
                match self {$(
                    FrameGraphNode::$name(node) => {
                        node.execute(render_data, swapchain_size, cmd_buffer, render_objects)
                    }
                )*}
            }

            /// Get the resources that this node writes to
            pub(in crate::render::frame_graph) fn write_resources<
                T,
                F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceWriteUsage)]) -> T,
            >(
                &self,
                f: F,
            ) -> T {
                match self {$(
                    FrameGraphNode::$name(node) => node.write_resources(f),
                )*}
            }

            /// Create the persistent objects that are used by nodes
            pub(in crate::render) fn create_objects(
                pipelines: &mut Vec<Pipeline>,
                fullscreen_quad: &Arc<Shader>,
                swapchain_format: VulkanFormat,
                device: &VulkanDevice,
            ) -> Result<()> {
                $(
                    $type::create_objects(
                        pipelines,
                        fullscreen_quad,
                        swapchain_format,
                        device
                    )?;
                )*

                Ok(())
            }
        }

        $(
            impl From<$type> for FrameGraphNode {
                fn from(node: $type) -> Self {
                    FrameGraphNode::$name(node)
                }
            }
        )*
    };
}
pub(in crate::render::frame_graph::nodes) use nodes;
