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
        pub(in crate::render::frame_graph) enum FrameGraphNode {$(
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

                materials: &SlotMap<RenderMaterial>,
            ) {
                match self {$(
                    FrameGraphNode::$name(node) => {
                        node.execute(render_data, swapchain_size, cmd_buffer, materials)
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
