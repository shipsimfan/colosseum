#![feature(associated_type_defaults)]

mod app;
mod camera;
mod game;
mod mesh_renderer;
mod shader;
mod transform;
mod window;

pub use app::*;
pub use camera::*;
pub use game::*;
pub use mesh_renderer::*;
pub use shader::{Shader, Vertex};
pub use transform::*;
pub use window::Window;

pub use alexandria::{Input, Matrix, Vector2, Vector3, Vector4};
