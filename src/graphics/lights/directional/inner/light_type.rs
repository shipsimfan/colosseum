use crate::{
    graphics::{context::LightType, lights::DirectionalLightInner},
    math::{Color3f, Vector3f},
};

/// The GPU representation of a directional light
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics) struct DirectionalLightGPU {
    /// The direction of the light
    direction: Vector3f,

    /// The color of the light
    color: Color3f,
}

impl LightType for DirectionalLightInner {
    type GPU = DirectionalLightGPU;

    fn to_gpu(&self) -> Self::GPU {
        DirectionalLightGPU {
            direction: self.direction,
            color: self.color,
        }
    }

    fn update(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }
}
