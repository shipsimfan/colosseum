//! The graphics subsystem

pub mod lights;

mod adapter;
mod camera;
mod context;
mod material;
mod mesh;
mod mesh_renderer;
mod settings;
mod shader;
mod vertex;

mod util;

pub use adapter::{Adapter, Output, OutputResolution};
pub use camera::{Camera, CameraHandle, CameraProjection};
pub use context::GraphicsContext;
pub use material::{Material, MaterialHandle};
pub use mesh::{Mesh, MeshInner, MeshPrimitives};
pub use mesh_renderer::{MeshRenderer, MeshRendererHandle};
pub use settings::{DisplayMode, GraphicsSettings};
pub use shader::{Shader, ShaderInner, ShaderSource};
pub use vertex::Vertex;
