#![feature(associated_type_defaults)]

mod app;
mod camera;
mod game;
mod mesh_renderer;
mod shader;
mod sprite;
mod texture;
mod transform;
mod transform_2d;
mod window;

pub use app::*;
pub use camera::*;
pub use game::*;
pub use mesh_renderer::*;
pub use shader::{Shader, Vertex};
pub use sprite::*;
pub use texture::Texture;
pub use transform::*;
pub use transform_2d::*;
pub use window::Window;

pub use alexandria::{Input, Matrix, SampleType, StateTrackingInput, Vector2, Vector3, Vector4};
