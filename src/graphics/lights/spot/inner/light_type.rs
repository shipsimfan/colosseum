use crate::{
    graphics::{context::LightType, lights::SpotLightInner},
    math::{Color3f, Vector3f},
};

/// The GPU representation of a point light
#[repr(C)]
#[derive(Clone, Copy)]
pub(in crate::graphics) struct SpotLightGPU {
    /// The position of the light
    position: Vector3f,

    /// The distance the light shines
    distance: f32,

    /// The direction of the light
    direction: Vector3f,

    /// The cut-off angle of the light
    cut_off: f32,

    /// The color of the light
    color: Color3f,

    /// The brightness of the light
    brightness: f32,
}

impl LightType for SpotLightInner {
    type GPU = SpotLightGPU;

    fn to_gpu(&self) -> Self::GPU {
        SpotLightGPU {
            position: self.position,
            distance: self.distance,
            direction: self.direction,
            cut_off: self.cut_off,
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
