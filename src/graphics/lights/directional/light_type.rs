use crate::{
    graphics::{context::LightType, lights::DirectionalLight},
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

    /// The brightness of the light
    brightness: f32,
}

impl LightType for DirectionalLight {
    type GPU = DirectionalLightGPU;

    fn to_gpu(&self) -> Self::GPU {
        DirectionalLightGPU {
            direction: self.direction,
            color: self.color,
            brightness: self.brightness,
        }
    }

    fn update(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }
}
