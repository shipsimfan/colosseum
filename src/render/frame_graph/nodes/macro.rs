macro_rules! nodes {
    [
        unsampled: [$(
            $(#[$meta: meta])*
            $module: ident::$name: ident($type: ident),
        )*],
        sampled: [$(
            $(#[$sampled_meta: meta])*
            $sampled_module: ident::$sampled_name: ident($sampled_type: ident),
        )*]
    ] => {
        $(
            mod $module;

            pub(in crate::render::frame_graph) use $module::$type;
        )*
        $(
            mod $sampled_module;

            pub(in crate::render::frame_graph) use $sampled_module::$sampled_type;
        )*

        /// A single node in the frame graph
        #[derive(Debug)]
        #[allow(private_interfaces)]
        pub(in crate::render) enum FrameGraphNode {
            $(
                $(#[$meta])*
                $name($type),
            )*
            $(
                $(#[$sampled_meta])*
                $sampled_name($sampled_type),
            )*
        }

        impl FrameGraphNode {
            /// Execute this node, performing the rendering operations associated with it
            pub(in crate::render::frame_graph) fn execute(
                &self,
                render_data: &RenderData,
                render_objects: &RenderObjects,
                resources: &FrameGraphResources,
                cmd_buffer: &mut VulkanCommandBuffer,
            ) {
                match self {
                    $(FrameGraphNode::$name(node) => {
                        node.execute(render_data, render_objects, resources, cmd_buffer)
                    })*
                    $(FrameGraphNode::$sampled_name(node) => {
                        node.execute(render_data, render_objects, resources, cmd_buffer)
                    })*
                }
            }

            /// Get the usage types for the resources that this node uses
            pub(in crate::render::frame_graph) fn usages<
                T,
                F: FnOnce(&[(FrameGraphResourceId, FrameGraphResourceUsage)]) -> T,
            >(
                &self,
                f: F,
            ) -> T {
                match self {
                    $(FrameGraphNode::$name(node) => node.usages(f),)*
                    $(FrameGraphNode::$sampled_name(node) => node.usages(f),)*
                }
            }

            /// Create the persistent objects that are used by nodes
            pub(in crate::render) fn create_objects(
                fixed_render_objects: &mut FixedRenderObjects,
                swapchain_format: VulkanFormat,
                device: &VulkanDevice,
            ) -> Result<()> {
                $(
                    $type::create_objects(
                        fixed_render_objects,
                        swapchain_format,
                        device
                    )?;
                )*
                $(
                    $sampled_type::create_objects(
                        fixed_render_objects,
                        swapchain_format,
                        device
                    )?;
                )*

                Ok(())
            }

            /// Create per-frame descriptor sets for this node
            pub(in crate::render) fn create_descriptor_sets(
                fixed_render_objects: &FixedRenderObjects,
                descriptor_pool: &mut VulkanDescriptorPool,
                descriptor_sets: &mut Vec<VulkanDescriptorSet>,
            ) -> Result<()> {
                $(
                    $sampled_type::create_descriptor_sets(
                        fixed_render_objects,
                        descriptor_pool,
                        descriptor_sets,
                    )?;
                )*

                Ok(())
            }

            /// Update the descriptor sets for this node
            pub(in crate::render::frame_graph) fn update_descriptor_sets(
                &self,
                render_objects: &RenderObjects,
                resources: &FrameGraphResources,
                device: &VulkanDevice,
            ){
                match self {
                    $(FrameGraphNode::$sampled_name(node) => {
                        node.update_descriptor_sets(render_objects, resources, device)
                    })*
                    _ => {}
                }
            }
        }

        $(
            impl From<$type> for FrameGraphNode {
                fn from(node: $type) -> Self {
                    FrameGraphNode::$name(node)
                }
            }
        )*

        $(
            impl From<$sampled_type> for FrameGraphNode {
                fn from(node: $sampled_type) -> Self {
                    FrameGraphNode::$sampled_name(node)
                }
            }
        )*
    };
}
pub(in crate::render::frame_graph::nodes) use nodes;
