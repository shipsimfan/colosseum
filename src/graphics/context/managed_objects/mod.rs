use crate::graphics::{CameraInner, Material, MaterialInner, Shader};
use std::{cell::RefCell, num::NonZeroU32, rc::Rc};

mod lights;

mod create;
mod get;
mod new;

pub(in crate::graphics) use lights::Lights;

/// The objects which are created by the game but managed by the engine
pub(in crate::graphics::context) struct ManagedGraphicsObjects {
    /// The cameras that have been registered
    cameras: Rc<RefCell<Vec<Rc<RefCell<CameraInner>>>>>,

    /// The materials that have been registered
    opaque_materials: Rc<RefCell<Vec<Rc<RefCell<MaterialInner>>>>>,

    /// The set of lights in the scene
    lights: Lights,

    /// The default lit shader
    default_lit_shader: Shader,

    /// The default unlit shader
    default_unlit_shader: Shader,

    /// The default lit material
    default_lit_material: Material,

    /// The default unlit material
    default_unlit_material: Material,

    /// The ID to assign the next shader
    next_shader_id: NonZeroU32,
}
