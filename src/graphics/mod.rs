//! The graphics subsystem

mod adapter;
mod context;
mod managed_objects;
mod mesh;
mod settings;
mod shader_source;
mod vertex;

mod util;

pub use adapter::{Adapter, Output, OutputResolution};
pub use colosseum_macros::{compile_shader, compile_shader_file};
pub use context::{AntiAliasing, GraphicsContext, PostProcessingShader};
pub use managed_objects::*;
pub use mesh::{Mesh, MeshInner, MeshPrimitives};
pub use settings::{DisplayMode, GraphicsSettings};
pub use shader_source::ShaderSource;
pub use vertex::Vertex;
