mod camera;
mod lights;
mod material;
mod mesh_renderer;

mod new;

pub use camera::*;
pub use lights::*;
pub use material::*;
pub use mesh_renderer::*;

/// The graphics objects which are managed by the engine
pub struct ManagedGraphicsObjects {
    /// The set of registered [`Camera`]s
    pub cameras: Cameras,

    /// The set of registered opaque [`Material`]s
    pub opaque_materials: Materials,

    /// The set of registered [`MeshRenderer`]s
    pub mesh_renderers: MeshRenderers,

    /// The lights in the scene
    pub lights: Lights,
}
