use crate::graphics::{CameraInner, Material, MaterialInner, Shader};
use std::{cell::RefCell, num::NonZeroU32, rc::Rc};

mod create;
mod get;
mod new;

/// The objects which are created by the game but managed by the engine
pub(in crate::graphics::context) struct ManagedGraphicsObjects {
    /// The cameras that have been registered
    cameras: Rc<RefCell<Vec<Rc<RefCell<CameraInner>>>>>,

    /// The materials that have been registered
    opaque_materials: Rc<RefCell<Vec<Rc<RefCell<MaterialInner>>>>>,

    /// The default shader
    default_shader: Shader,

    /// The default material
    default_material: Material,

    /// The ID to assign the next shader
    next_shader_id: NonZeroU32,
}
