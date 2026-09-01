macro_rules! nodes {
    [
        simple: [$(
            $(#[$meta: meta])*
            $module: ident::$name: ident($type: ident),
        )*],
        data_buffer: [$(
            $(#[$data_buffer_meta: meta])*
            $data_buffer_module: ident::$data_buffer_name: ident($data_buffer_type: ident),
        )*],
        post_process: [$(
            $(#[$post_process_meta: meta])*
            $post_process_module: ident::$post_process_name: ident($post_process_type: ident),
        )*]
    ] => {
        $(
            mod $module;

            pub(in crate::render::frame_graph) use $module::$type;
        )*
        $(
            mod $data_buffer_module;

            pub(in crate::render::frame_graph) use $data_buffer_module::$data_buffer_type;
        )*
        $(
            mod $post_process_module;

            pub(in crate::render::frame_graph) use $post_process_module::$post_process_type;
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
                $(#[$data_buffer_meta])*
                $data_buffer_name($data_buffer_type),
            )*
            $(
                $(#[$post_process_meta])*
                $post_process_name($post_process_type),
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
                    $(FrameGraphNode::$data_buffer_name(node) => {
                        node.execute(render_data, render_objects, resources, cmd_buffer)
                    })*
                    $(FrameGraphNode::$post_process_name(node) => {
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
                    $(FrameGraphNode::$data_buffer_name(node) => node.usages(f),)*
                    $(FrameGraphNode::$post_process_name(node) => node.usages(f),)*
                }
            }

            /// Create the persistent objects that are used by nodes
            pub(in crate::render) fn create_fixed_objects(
                fixed_render_objects: &mut FixedRenderObjects,
                swapchain_format: VulkanFormat,
                device: &VulkanDevice,
            ) -> Result<()> {
                $(
                    $type::create_fixed_objects(
                        fixed_render_objects,
                        swapchain_format,
                        device
                    )?;
                )*
                $(
                    $data_buffer_type::create_fixed_objects(
                        fixed_render_objects,
                        swapchain_format,
                        device
                    )?;
                )*
                $(
                    $post_process_type::create_fixed_objects(
                        fixed_render_objects,
                        swapchain_format,
                        device
                    )?;
                )*

                Ok(())
            }

            /// Create per-frame descriptor sets for this node
            pub(in crate::render) fn create_per_frame_objects(
                mut per_frame_objects: PerFrameObjectBuilder,
            ) -> Result<()> {
                $($type::create_per_frame_objects(&mut per_frame_objects,)?;)*
                $($data_buffer_type::create_per_frame_objects(&mut per_frame_objects)?;)*
                $($post_process_type::create_per_frame_objects(&mut per_frame_objects)?;)*

                Ok(())
            }

            /// Copy data from staging buffers to device local buffers
            pub(in crate::render) fn copy_data(
                render_data: &RenderData,
                device_buffers: &mut [DeviceDataBuffer],
                cmd_buffer: &mut VulkanCommandBuffer,
                device: &VulkanDevice,
                memory_properties: &VulkanAdapterMemoryProperties,
            ) -> Result<()> {
                $($data_buffer_type::copy_data(render_data, device_buffers, cmd_buffer, device, memory_properties)?;)*

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
                    $(FrameGraphNode::$post_process_name(node) => {
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
            impl From<$data_buffer_type> for FrameGraphNode {
                fn from(node: $data_buffer_type) -> Self {
                    FrameGraphNode::$data_buffer_name(node)
                }
            }
        )*

        $(
            impl From<$post_process_type> for FrameGraphNode {
                fn from(node: $post_process_type) -> Self {
                    FrameGraphNode::$post_process_name(node)
                }
            }
        )*
    };
}
pub(in crate::render::frame_graph::nodes) use nodes;
