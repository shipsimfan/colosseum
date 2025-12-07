use crate::{
    graphics::{PointLight, managed_objects::lights::LightType},
    math::{Color3f, Vector3f},
};

/// The GPU representation of a point light
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics::managed_objects::lights) struct PointLightGPU {
    /// The position of the light
    position: Vector3f,

    /// The radius of the light
    radius: f32,

    /// The color of the light
    color: Color3f,

    /// The brightness of the light
    brightness: f32,
}

impl LightType for PointLight {
    type GPU = PointLightGPU;

    fn to_gpu(&self) -> Self::GPU {
        PointLightGPU {
            position: self.position,
            radius: self.radius,
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
