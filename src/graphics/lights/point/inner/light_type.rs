use crate::{
    graphics::{context::LightType, lights::PointLightInner},
    math::{Color3f, Vector3f},
};

/// The GPU representation of a point light
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics) struct PointLightGPU {
    /// The position of the light
    position: Vector3f,

    /// The radius of the light
    radius: f32,

    /// The brightness of the light
    brightness: f32,

    /// The color of the light
    color: Color3f,
}

impl LightType for PointLightInner {
    type GPU = PointLightGPU;

    fn to_gpu(&self) -> Self::GPU {
        PointLightGPU {
            position: self.position,
            radius: self.radius,
            brightness: self.brightness,
            color: self.color,
        }
    }

    fn update(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }
}
