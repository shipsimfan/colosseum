//! The graphics subsystem

mod adapter;
mod context;
mod managed_objects;
mod mesh;
mod settings;
mod shader;
mod vertex;

mod util;

pub use adapter::{Adapter, Output, OutputResolution};
pub use context::GraphicsContext;
pub use managed_objects::*;
pub use mesh::{Mesh, MeshInner, MeshPrimitives};
pub use settings::{DisplayMode, GraphicsSettings};
pub use shader::{Shader, ShaderInner, ShaderSource};
pub use vertex::Vertex;
