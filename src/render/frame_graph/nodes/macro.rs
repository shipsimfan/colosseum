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
